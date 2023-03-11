use lazy_static::lazy_static;
use rand::{thread_rng, Rng, SeedableRng};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

lazy_static! {
    static ref CJK_REGEX: Regex = Regex::new(r"\p{Unified_Ideograph}").unwrap();
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
    // filter_processed().unwrap();
    reservoir_sample_lines(50).unwrap();
}

fn filter_processed() -> std::io::Result<()> {
    let dir_entries = std::fs::read_dir("processed")?;
    let output_file = File::create("sentences.txt")?;
    let mut writer = BufWriter::new(output_file);
    for entry in dir_entries {
        let file_path = entry.unwrap().path();
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let (sentence, _) = split_line_at_last_tab(&line);
            let num_cjk = count_matching_chars(sentence, &CJK_REGEX);
            let num_total = sentence.chars().count();
            if num_cjk >= 5 && num_cjk > ((num_total as f32 * 0.8).round() as usize) {
                writer.write_all((filter_irrelevant_chars(sentence) + "\n").as_bytes())?;
            }
        }
    }
    writer.flush()
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
