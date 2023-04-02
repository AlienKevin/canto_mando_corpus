import cantoseg

with open("sentences_typo_fixed_uniq.txt", "r") as input_file, open("sentences_typo_fixed_uniq_word_seg.txt", "w+") as output_file:
    for sentence in input_file.read().splitlines():
        output_file.write(" ".join(cantoseg.cut(sentence.replace(" ", ""))) + "\n")
