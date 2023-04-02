from StarCC import PresetConversion
convert = PresetConversion(src='cn', dst='hk', with_phrase=False, use_seg=True)
with open("LCCC_sentences_cn.txt", "r") as input_file, open("LCCC_sentences_hk.txt", "w+") as output_file:
    for sentence in input_file.read().splitlines():
        output_file.write(convert(sentence) + "\n")
