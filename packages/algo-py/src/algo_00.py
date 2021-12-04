def most_frequent(arr):
    ret = None
    counter = {}
    max_count = -1
    for n in arr:
        counter.setdefault(n, 0)
        counter[n] += 1
        if counter[n] > max_count:
            max_count = counter[n]
            ret = n
    return ret
