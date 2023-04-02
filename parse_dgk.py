from ordered_set import OrderedSet
import filter_characters
from StarCC import PresetConversion

convert = PresetConversion(src='cn', dst='hk', with_phrase=False, use_seg=True)
sentences = OrderedSet()

with open("dgk_conv/dgk_shooter_z.conv", "r", encoding='utf-8', errors='ignore') as input_file:
    for line in input_file.read().splitlines():
        if line.startswith("E"):
            continue
        else:
            sentence = " ".join([token for token in line[2:].split("/") if token != ''])
            sentences.add(sentence)

with open("dgk_conv/dgk_sentences.txt", "w+") as output_file:
    for sentence in sentences:
        sentence = convert(sentence)
        if filter_characters.is_accepted_sentence(sentence):
            output_file.write(sentence + "\n")
