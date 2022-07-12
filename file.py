def file_to_array(filename: str):
    words = []

    with open(filename) as file:
        for line in file:
            words.append(line.strip())
    return words