#!/usr/bin/env python3
import fileinput

s=0
for line in fileinput.input():
    line = line.strip()
    mid = len(line)//2
    a, b = line[:mid], line[mid:]
    c = set(a).intersection(b).pop()
    if c.isupper():
        v = ord(c) - ord('A') + 1 + 26
    else:
        v = ord(c) - ord('a') + 1
    s+=v
print(s)
