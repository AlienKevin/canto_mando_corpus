import json

with open("LCCC-base_train.json", "r") as input_file, open("LCCC_sentences.txt", "w+") as output_file:
    conversations = json.load(input_file)
    for conversation in conversations:
        for sentence in conversation:
            if len(sentence) >= 5:
                output_file.write(sentence + "\n")
