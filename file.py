def file_to_list(filename: str):
    words = []

    with open(filename) as file:
        for line in file:
            words.append(line.strip())
    return words