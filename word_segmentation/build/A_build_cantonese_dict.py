from collections import Counter
from os import path
import pycantonese as pc

if __name__ == '__main__':
    HERE = path.abspath(path.dirname(__file__))

    corpus = pc.hkcancor()
    c = Counter()

    for token in corpus.tagged_words():
        if token.pos.isalpha():
            c[token.word, token.pos.lower()] += 1

    with open(path.join(HERE, 'dict_cantonese.txt'), 'w', encoding='utf8') as f:
        for (word, pos), freq in c.most_common():
            print(word, freq, pos, file=f)
