#!/usr/bin/env python3
data = [eval(l) for l in open(0) if l.strip()]
pairs = zip(data[::2], data[1::2])
cmp = lambda x, y: (x > y) - (x < y)

def comparison(left, right):
    l_isint = isinstance(left, int)
    r_isint = isinstance(right, int)

    if l_isint and r_isint:
        return cmp(left, right)
    elif l_isint:
        left = [left]
    elif r_isint:
        right = [right]

    for l,r in zip(left, right):
        diff = comparison(l,r)
        if diff != 0:
            return diff

    return cmp(len(left), len(right))

a = sum(i for i, p in enumerate(pairs, 1) if comparison(*p) == -1)
print(a)
