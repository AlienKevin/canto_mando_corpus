zh_words = set()
with open('zh_tw_words.txt', 'r') as zh_small_file:
    for line in zh_small_file.read().splitlines():
        zh_words.add(line.split(" ")[0])

# with open('zh_cn_and_tw_words.txt') as zh_big_file:
#     for word in zh_big_file.read().splitlines():
#         zh_words.add(word)

with open('yue_words.txt', 'r') as yue_file:
    yue_words = []
    for line in yue_file.read().splitlines():
        if line.endswith("\tcolloquial"):
            yue_words.append(line.removesuffix("\tcolloquial"))
        else:
            yue_entry = []
            [variants, pos] = line.split("\t")
            for variant in variants.split(","):
                if not variant in zh_words:
                    yue_entry.append(variant)
            if len(yue_entry) > 0:
                yue_words.append(",".join(yue_entry) + "\t" + pos)
    with open('marked_yue_words.txt', 'w+') as output_file:
        output_file.writelines("%s\n" % word for word in yue_words)
