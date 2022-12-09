#!/usr/bin/env python3
lines = open(0).read().splitlines()
m = [[int(c) for c in l] for l in lines]
n = len(m)

visible = set()
for o in range(n):
    highest = [-1, -1, -1, -1]
    for i in range(n):
        rev = n - 1 - i
        for dr,dc,h in (o,i,0),(o,rev,1),(i,o,2),(rev,o,3):
            v = m[dr][dc]
            if v > highest[h]:
                visible.add((dr,dc))
                highest[h] = v

print(len(visible))
