#!/usr/bin/env python3.11
from interval_tree import insert, inOrder, merge

root = None
#y = 2000000
y = 2655411
#y = 3166538
# 5968076 is too high
# 7257164 is too high
for line in open(0):
    words = line.strip().split()
    sx=int(words[2].split('=')[1][:-1])
    sy=int(words[3].split('=')[1][:-1])
    bx=int(words[8].split('=')[1][:-1])
    by=int(words[9].split('=')[1])
    dist = abs(sx-bx)+abs(sy-by)
    if sy - dist <= y <= sy + dist:
        to = dist - abs(sy-y)
        root = insert(root, sx - to, sx + to)
#inOrder(root)
#print()
#print(root.stat())
x = merge(root, [])
print()
print(x)
print(sum(d[1]-d[0] for d in x))
