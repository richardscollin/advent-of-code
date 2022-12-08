#!/usr/bin/env python3
from itertools import product
from pprint import pprint
lines = [l.strip() for l in open(0).readlines()]
n = len(lines)
m = [int(c) for l in lines for c in l]
g = [1]*n + ([1]+[0]*(n-2)+[1])*(n-2) + [1]*n

def idx(r,c,n):
    return r*n+c

def all_lt(l, v):
    return all(map(lambda x: x < v, l))

for r,c in product(range(1,n-1),repeat=2):
    top = m[idx(0,c,n):idx(r,c,n):n]
    bot = m[idx(r+1,c,n):idx(n,c,n):n]
    left = m[idx(r,0,n):idx(r,c,n)]
    right = m[idx(r,c+1,n):idx(r,n,n)]
    v = m[idx(r,c,n)]
    x = [all_lt(d, v) for d in (top, bot, left, right)]
    g[idx(r,c,n)] = int(any(x))

def print_grid(m):
    for i in range(n):
        print(m[(i*n):(i+1)*n])

print_grid(g)
print()
print_grid(m)
print(sum(g))
