import json
import cantoseg
import jieba

from StarCC import PresetConversion
convert = PresetConversion(src='cn', dst='hk', with_phrase=False, use_seg=True)

with open("yue_zh.json", "r") as input_file,\
    open("yue_valid.txt", "w+") as yue_valid_file,\
    open("yue_test.txt", "w+") as yue_test_file,\
    open("zh_valid.txt", "w+") as zh_valid_file,\
    open("zh_test.txt", "w+") as zh_test_file:
    lines = input_file.read().splitlines()
    for i, line in enumerate(lines):
        translation = json.loads(line)["translation"]
        yue = " ".join(cantoseg.cut(translation["yue"].replace(" ", "")))
        zh = convert(" ".join(jieba.cut(translation["zh"])))
        if i < len(lines) / 2:        
            yue_valid_file.write(yue + "\n")
            zh_valid_file.write(zh + "\n")
        else:
            yue_test_file.write(yue + "\n")
            zh_test_file.write(zh + "\n")
