#!/usr/bin/env python3
from operator import mul
from itertools import product,takewhile
from functools import reduce
lines = [l.strip() for l in open(0).readlines()]
n = len(lines)
m = [int(c) for l in lines for c in l]
g = [0] * (n*n)

def idx(r,c,n):
    return r*n+c

def score(trees, tree_height):
    seen = 0
    for t in trees:
        seen += 1
        if t >= tree_height:
            break
    return seen

for r,c in product(range(0,n),repeat=2):
    top = m[idx(0,c,n):idx(r,c,n):n][::-1]
    bot = m[idx(r+1,c,n):idx(n,c,n):n]
    left = m[idx(r,0,n):idx(r,c,n)][::-1]
    right = m[idx(r,c+1,n):idx(r,n,n)]
    height = m[idx(r,c,n)]
    scores = [score(d, height) for d in (top, bot, left, right)]
    g[idx(r,c,n)] = reduce(mul, scores)

print(max(g))
