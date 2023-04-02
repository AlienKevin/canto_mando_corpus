from ordered_set import OrderedSet
import filter_characters
from StarCC import PresetConversion

convert = PresetConversion(src='cn', dst='hk', with_phrase=False, use_seg=True)
sentences = OrderedSet()

with open("douban/train.txt", "r") as input_file:
    for line in input_file.read().splitlines():
        for sentence in line[2:].split("\t"):
            sentences.add(sentence)

with open("douban/douban_sentences.txt", "w+") as output_file:
    for sentence in sentences:
        sentence = convert(sentence)
        if not "_url_" in sentence and filter_characters.is_accepted_sentence(sentence):
            output_file.write(sentence + "\n")
