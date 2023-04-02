import classifier
from filter_characters import is_accepted_sentence

import re


# Filter sensitive words
lan_exceptions = ["撚下人",
"撚人",
"撚化",
"撚手",
"撚手小菜",
"撚狗",
"撚疼",
"撚菜",
"撚雀",
"撚頭",
"撚頸",
"毒撚",
"窮撚",
"笨撚",
"耶撚",
"道德撚",
]

lan_exceptions_pattern = "|".join(lan_exceptions)
other_sensitive_pattern = "柒|𨳊|鳩|支那|中共|共產|中華民國|民建聯|民主黨|林鄭|黑警|泛民|左膠|黃絲|黃屍|建制|藍絲|藍屍|黨鐵|區議會|殘體字|六四|寵寵寵"

def is_sensitive(sentence: str) -> bool:
    return "撚" in re.sub(lan_exceptions_pattern, "", sentence) or re.search(other_sensitive_pattern, sentence)

assert not is_sensitive("佢係個道德撚嚟嘅。")
assert is_sensitive("個笨撚唔撚識讀嗰個字。")


with open("LCCC-base-split/LCCC_sentences_hk_8M.txt", "w+") as output_file:
    i = 0
    for line in reversed(list(open("LCCC-base-split/LCCC_sentences_hk.txt"))):
        if is_accepted_sentence(line):
            output_file.write(line)
            i += 1
            if i == 8 * (10 ** 6):
                output_file.flush()
                break

with open("sentences_8M_word_seg.txt", "w+") as output_file:
    i = 0
    for line in reversed(list(open("sentences_typo_fixed_uniq_word_seg.txt"))):
        if is_accepted_sentence(line) and not is_sensitive(line)\
            and classifier.judge(line) == 'cantonese':
            output_file.write(line)
            i += 1
            if i == 8 * (10 ** 6):
                output_file.flush()
                break
