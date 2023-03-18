canto_mando = {}
with open("canto_mando_words.txt") as canto_mando_file:
    for line in canto_mando_file.read().splitlines():
        [canto_word, mando_word] = line.split("\t")
        canto_mando[canto_word] = mando_word

assert("瞓覺" in canto_mando)

canto_without_translation = []

with open("marked_yue_words.txt") as yue_file:
    for line in yue_file.read().splitlines():
        yue_variants = line.split("\t")[0].split(",")
        found_translation = False
        for variant in yue_variants:
            if variant in canto_mando:
                print("{}\t{}".format(",".join(yue_variants), canto_mando[variant]))
                found_translation = True
                break
        if not found_translation:
            canto_without_translation.append(",".join(yue_variants))

# for canto_word in canto_without_translation:
#     print("{}\t?".format(canto_word))
