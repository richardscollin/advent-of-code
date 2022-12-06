#!/usr/bin/env python3
import fileinput

s=0
l=[line.strip() for line in fileinput.input()]

for i in range(0,len(l),3):
    a,b,c=l[i],l[i+1],l[i+2]
    c = set(a).intersection(b).intersection(c).pop()
    if c.isupper():
        v = ord(c) - ord('A') + 1 + 26
    else:
        v = ord(c) - ord('a') + 1
    s+=v
print(s)
