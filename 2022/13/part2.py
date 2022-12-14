#!/usr/bin/env python3
from pprint import pprint
from functools import cmp_to_key
data = [eval(l) for l in open(0) if l.strip()]
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

divider_2 = [[2]]
divider_6 = [[6]]
data.append(divider_2)
data.append(divider_6)
data.sort(key=cmp_to_key(comparison))

ans = 1
ans *= (data.index(divider_2)+1)
ans *= (data.index(divider_6)+1)

print(ans)
