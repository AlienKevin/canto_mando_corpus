import re

simplified_chars = set()

with open("STCharacters.txt", "r") as input_file:
    for line in input_file.read().splitlines():
        [simp, trad] = line.split("\t")
        if not " " in trad:
            simplified_chars.add(simp)

with open("HKVariants.txt", "r") as input_file:
    for line in input_file.read().splitlines():
        [trad, hk_variants] = line.split("\t")
        for variant in hk_variants.split(" "):
            if variant in simplified_chars:
                simplified_chars.remove(variant)


punctuations = { '$', '%', '…', '—', '～',
'~', '`', '!', '(', ')', '-', '_', '{', '}', '[', ']', '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/',
'！', '：', '；', '“', '”', '‘', '’', '【', '】', '（', '）', '「', '」', '﹁', '﹂',
'『', '』', '《', '》', '？', '，', '。', '、', '／', '＋', '〈', '〉', '︿', '﹀',
'［', '］', '‧' }

chinese_char_pattern = re.compile(r"[\u4e00-\u9fff]")
alphanumeric_pattern = re.compile(r"[a-zA-Z0-9 ]")

repeated_4_times_pattern = re.compile(r"(.)\1{3}")

def is_accepted_sentence(sentence: str) -> bool:
    sentence = sentence.strip()
    # # Has to contain at least 5 Chinese characters
    # if len("".join(chinese_char_pattern.findall(sentence))) < 5:
    #     return False
    if repeated_4_times_pattern.match(sentence.replace(" ", "")):
        return False
    for c in sentence:
        is_accepted_char = (chinese_char_pattern.match(c) and not c in simplified_chars) or c in punctuations or alphanumeric_pattern.match(c)
        if not is_accepted_char:
            return False
    return True
