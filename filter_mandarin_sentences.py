import importlib
import filter_characters
importlib.reload(filter_characters)

assert (not filter_characters.is_accepted_sentence("哈哈哈哈"))

with open("LCCC-base-split/LCCC_sentences_hk_8M.txt", "w+") as output_file:
    i = 0
    for line in reversed(list(open("LCCC-base-split/LCCC_sentences_hk.txt"))):
        if filter_characters.is_accepted_sentence(line):
            output_file.write(line)
            i += 1
            if i == 8 * (10 ** 6):
                output_file.flush()
                break
