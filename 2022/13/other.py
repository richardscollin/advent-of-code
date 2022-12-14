#!/usr/bin/env python3.11
data = [eval(l) for l in open(0) if l.strip()]
pairs = zip(data[::2], data[1::2])

def comparison(left, right):
    match left, right:
        case int(), int():
            return (left > right) - (left < right)
        case int(), list():
            return comparison([left], right)
        case list(), int():
            return comparison(left, [right])
        case list(), list():
            for l,r in zip(left, right):
                diff = comparison(l,r)
                if diff != 0:
                    return diff
            return comparison(len(left), len(right)) # base case

a = sum(i for i, p in enumerate(pairs, 1) if comparison(*p) == -1)
print(a)
