# "吖", "aa1", "啊", "吖啶", "aa1", "ding6", 

import ast
import re

# Open the file for reading
with open('canto_mando_dict.txt', 'r') as f:
    # Read the contents of the file as a string
    contents = f.read()
    # Use ast.literal_eval to safely evaluate the string as a Python list
    dict_list = ast.literal_eval(contents)

def is_jyutping(word: str) -> bool:
    return re.match(r'^[a-z]+[1-6]$', word) is not None

i = 0
while i < len(dict_list):
    word = dict_list[i]
    i += 1
    # skip jyutping
    while i < len(dict_list) and is_jyutping(dict_list[i]):
        i += 1
    if i + 1 < len(dict_list) and not is_jyutping(dict_list[i + 1]):
        definition = dict_list[i]
        print("{}\t{}".format(word, definition if '，' in word else definition.split('，')[0]))
        i += 1
