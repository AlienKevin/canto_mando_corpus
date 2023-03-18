# LIHKG Corpus Statistics

There are 108,647,275 sentences in total, 5% (5,378,375) of those are filtered because they contain "bad words" including swears, sexual language, reference to drugs. The list of bad words is generated from all entries labeled vulgar (粗俗) and nsfw (黃賭毒) in words.hk (Mar 12 2023 version). See `bad_words.txt` for details.

For the rest of the sentences, 40% are markedly Cantonese, while 47% are neutral in the sense that there's no markedly Cantonese nor Mandarin features present. It's safe to assume that at least 80% of the sentences are in Cantonese. See the table below for a detailed breakdown:

| Language | Sentences | Proportion |
| - | - | - |
| Cantonese | 43370662 | 40% |
| Cantonese mixed with Mandarin | 2216730 | 2% |
| Neutral | 50767873 | 47% |
| Mandarin | 6913635 | 6% |

![sentence lengths](sentence_lengths.png)

# Sources
|File| Source|
| - | - |
| zh_tw_words.txt | https://github.com/APCLab/jieba-tw |
| canto_mando_dict.txt | https://kaifangcidian.com/xiazai/cidian_zhyue-kfcd.zip |
| zh_cn_and_tw_words.txt | https://github.com/samejack/sc-dictionary |