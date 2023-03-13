import jieba
import logging
from os import path

HERE = path.abspath(path.dirname(__file__))

jieba.setLogLevel(logging.INFO)
jieba.set_dictionary(path.join(HERE, 'merged_dict.txt'))
jieba.initialize()

import jieba.posseg as pseg

def lcut(s):
    return (w.word for w in pseg.lcut(s))

def cut(s):
    return list(lcut(s))

if __name__ == '__main__':
    with open("../sample_segmentation.txt", "r") as f:
        lines = f.read().splitlines()
        num_lines = len(lines)
        wrong_results = []
        for line in lines:
            result = " ".join(cut(line.replace(" ", "")))
            if result != line:
                wrong_results.append((result, line))
        print(f"{num_lines - len(wrong_results)} correct / {num_lines} total\n")
        with open("segmentation_results.txt", "w+") as result_file:
            with open("segmentation_truth.txt", "w+") as truth_file:
                for result, line in wrong_results:
                    result_file.write(result + "\n")
                    truth_file.write(line + "\n")

