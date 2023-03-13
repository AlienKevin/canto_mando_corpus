zh_words = set()
with open('zh_tw_words.txt', 'r') as zh_file:
    for line in zh_file.read().splitlines():
        zh_words.add(line.split(" ")[0])

with open('yue_words.txt', 'r') as yue_file:
    yue_words = []
    for line in yue_file.read().splitlines():
        if '\t' in line:
            yue_words.append(line.split("\t")[0])
        else:
            yue_entry = []
            for yue_word in line.split(","):
                if not yue_word in zh_words:
                    yue_entry.append(yue_word)
            if len(yue_entry) > 0:
                yue_words.append(",".join(yue_entry))
    with open('marked_yue_words.txt', 'w+') as output_file:
        output_file.writelines("%s\n" % word for word in yue_words)
