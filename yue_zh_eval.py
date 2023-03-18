import cantoseg

canto_mando = {}
with open("marked_yue_to_zh_translation.txt") as file:
    for line in file.read().splitlines():
        [canto, mando] = line.split("\t")
        for variant in canto.split(","):
            canto_mando[variant] = mando

def translate_yue_word(w: str) -> str:
    if w in canto_mando:
        return canto_mando[w]
    else:
        num_has_translation = 0
        translation = ""
        for c in w:
            if c in canto_mando:
                num_has_translation += 1
                translation += canto_mando[c]
            else:
                translation += c
        if num_has_translation >= len(w) / 2:
            return translation
        else:
            return w           

def translate_yue_sentence(s: str) -> str:
    return " ".join([translate_yue_word(word) for word in cantoseg.cut(s)])

with open("yue_zh_sentences.txt", "r") as file:
    for line in	file.read().splitlines():
        [yue, zh] = line.split("\t")
        print("yue: {}\ntranslation: {}\nzh: {}\n".format(yue, translate_yue_sentence(yue), zh))
