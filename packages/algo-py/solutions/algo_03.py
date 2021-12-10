def max_substr(input):
    start = 0
    max_count = 0
    indexes = [-1 for _ in range(128)]
    for i in range(len(input)):
        c = ord(input[i])
        if indexes[c] >= start:
            start = indexes[c] + 1
        indexes[c] = i
        max_count = max(max_count, i - start + 1)
    return max_count
