#!/usr/bin/env python3
import fileinput

score=0
for line in fileinput.input():
    op,res = line.strip().split()
    op="ABC".index(op)
    res="XYZ".index(res)
    score+=res*3
    me=(op+res-1)%3
    score += (1+me)
print(score)
