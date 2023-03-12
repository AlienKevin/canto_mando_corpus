use dashmap::DashMap;
use lazy_static::lazy_static;
use plotters::prelude::*;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

lazy_static! {
    static ref CJK_REGEX: Regex = Regex::new(r"\p{Unified_Ideograph}").unwrap();
    static ref WORD_REGEX: Regex =
        Regex::new(r"[[:alnum:]]+|\p{Unified_Ideograph}|\p{Punct}+").unwrap();
    static ref PUNCS: HashSet<char> = {
        SHARED_PUNCS
            .union(&ENGLISH_PUNCS)
            .copied()
            .collect::<HashSet<char>>()
            .union(&CHINESE_PUNCS)
            .copied()
            .collect()
    };
    static ref SHARED_PUNCS: HashSet<char> =
        HashSet::from(['@', '#', '$', '%', '^', '&', '*', '·', '…', '‥', '—', '～']);
    static ref ENGLISH_PUNCS: HashSet<char> = {
        HashSet::from([
            '~', '`', '!', '(', ')', '-', '_', '{', '}', '[', ']', '|', '\\', ':', ';', '"', '\'',
            '<', '>', ',', '.', '?', '/',
        ])
    };
    static ref CHINESE_PUNCS: HashSet<char> = {
        HashSet::from([
            '！', '：', '；', '“', '”', '‘', '’', '【', '】', '（', '）', '「', '」', '﹁', '﹂',
            '『', '』', '《', '》', '？', '，', '。', '、', '／', '＋', '〈', '〉', '︿', '﹀',
            '［', '］', '‧',
        ])
    };
}

fn main() {
    filter_processed().unwrap();
    // reservoir_sample_lines(50).unwrap();
}

fn plot_sentence_lengths(lengths: DashMap<u32, u32>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("sentence_lengths.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(50)
        .margin(5)
        .caption("Sentence Length", ("sans-serif", 50.0))
        .build_cartesian_2d((5u32..30u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Proportion")
        .x_desc("Length")
        .axis_desc_style(("sans-serif", 25))
        .label_style(("sans-serif", 20))
        .draw()?;

    let total_count: u32 = lengths.iter().map(|entry| *entry.value()).sum();

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(lengths.iter().map(|entry| {
                (
                    *entry.key(),
                    (*entry.value() as f32 / total_count as f32 * 100.0).round() as u32,
                )
            })),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write histogram to file");

    Ok(())
}

fn get_bad_words() -> std::io::Result<HashSet<String>> {
    let file = File::open("bad_words.txt")?;
    let reader = BufReader::new(file);

    let mut bad_words = HashSet::new();
    for word in reader.lines() {
        bad_words.insert(word?);
    }

    Ok(bad_words)
}

fn filter_processed() -> std::io::Result<()> {
    let start_time = Instant::now();

    let bad_words = get_bad_words()?;

    let dir_entries = std::fs::read_dir("processed")?;
    let output_file = Arc::new(Mutex::new(File::create("sentences.txt")?));
    let num_mandarin = AtomicUsize::new(0);
    let num_mixed = AtomicUsize::new(0);
    let num_cantonese = AtomicUsize::new(0);
    let num_neutral = AtomicUsize::new(0);
    let num_bad = AtomicUsize::new(0);
    let num_lines = AtomicUsize::new(0);
    let sentence_lengths: DashMap<u32, u32> = DashMap::new();
    dir_entries
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(|file_path| {
            let mut sentences: Vec<String> = Vec::new();
            let file = File::open(&file_path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                num_lines.fetch_add(1, Ordering::Relaxed);
                let line = line.unwrap();
                let (sentence, language) = split_line_at_last_tab(&line);
                if bad_words.iter().any(|word| sentence.contains(word)) {
                    num_bad.fetch_add(1, Ordering::Relaxed);
                    continue;
                }
                match language {
                    "mandarin" => {
                        num_mandarin.fetch_add(1, Ordering::Relaxed);
                        continue;
                    }
                    "mixed" => {
                        num_mixed.fetch_add(1, Ordering::Relaxed);
                    }
                    "cantonese" => {
                        num_cantonese.fetch_add(1, Ordering::Relaxed);
                    }
                    "neutral" => {
                        num_neutral.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => panic!("Invalid language tag {}", language),
                };
                let num_cjk = count_matching_chars(sentence, &CJK_REGEX);
                let num_total = sentence.chars().count();
                if num_cjk >= 5 && num_cjk > ((num_total as f32 * 0.8).round() as usize) {
                    let sentence = filter_irrelevant_chars(sentence);
                    sentence_lengths
                        .entry(count_words(&sentence).try_into().unwrap())
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    sentences.push(sentence);
                }
            }
            let mut output_file = output_file.lock().unwrap();
            for sentence in sentences {
                output_file.write_all((sentence + "\n").as_bytes()).unwrap();
            }
            output_file.flush().unwrap();
        });
    let num_lines = num_lines.load(Ordering::Relaxed);
    let num_bad = num_bad.load(Ordering::Relaxed);
    let num_cantonese = num_cantonese.load(Ordering::Relaxed);
    let num_mixed = num_mixed.load(Ordering::Relaxed);
    let num_neutral = num_neutral.load(Ordering::Relaxed);
    let num_mandarin = num_mandarin.load(Ordering::Relaxed);
    println!("| Total | {} |", num_lines);
    println!(
        "| Bad | {} | {:.0}% |",
        num_bad,
        num_bad as f32 / num_lines as f32 * 100.0
    );
    println!(
        "| Cantonese | {} | {:.0}% |",
        num_cantonese,
        num_cantonese as f32 / num_lines as f32 * 100.0
    );
    println!(
        "| Cantonese mixed with Mandarin | {} | {:.0}% |",
        num_mixed,
        num_mixed as f32 / num_lines as f32 * 100.0
    );
    println!(
        "| Neutral | {} | {:.0}% |",
        num_neutral,
        num_neutral as f32 / num_lines as f32 * 100.0
    );
    println!(
        "| Mandarin | {} | {:.0}% |",
        num_mandarin,
        num_mandarin as f32 / num_lines as f32 * 100.0
    );
    plot_sentence_lengths(sentence_lengths).unwrap();
    let elapsed_time = start_time.elapsed().as_secs() / 60;
    println!("Took: {:?} minutes", elapsed_time);
    Ok(())
}

fn count_words(sentence: &str) -> usize {
    // Use the regular expression to split the sentence into tokens.
    let tokens = WORD_REGEX.find_iter(sentence);

    // Count the number of tokens.
    tokens.count()
}

fn filter_irrelevant_chars(text: &str) -> String {
    text.chars()
        .filter(|c| CJK_REGEX.is_match(&c.to_string()) || is_punc(*c) || c.is_ascii_alphanumeric())
        .collect()
}

fn count_matching_chars(text: &str, regex: &Regex) -> usize {
    text.chars()
        .filter(|c| regex.is_match(&c.to_string()))
        .count()
}

fn split_line_at_last_tab(line: &str) -> (&str, &str) {
    let chars = line.chars();
    let last_tab_index = chars.rev().position(|c| c == '\t').unwrap();
    let first_part = &line[0..(line.len() - last_tab_index - 1)];
    let second_part = &line[(line.len() - last_tab_index)..];
    (first_part, second_part)
}

fn is_punc(c: char) -> bool {
    PUNCS.contains(&c)
}

fn reservoir_sample_lines(num_lines: usize) -> std::io::Result<()> {
    let mut reservoir: Vec<String> = Vec::with_capacity(num_lines);
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut i = 0;

    let file = File::open("sentences.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if i < num_lines {
                reservoir.push(line);
            } else {
                let j = rng.gen_range(0..=i);
                if j < num_lines {
                    reservoir[j] = line;
                }
            }
            i += 1;
        }
    }

    let output_file = File::create("sample_sentences.txt")?;
    let mut writer = BufWriter::new(output_file);
    for line in reservoir {
        writer.write_all((line + "\n").as_bytes())?;
    }
    writer.flush()
}
