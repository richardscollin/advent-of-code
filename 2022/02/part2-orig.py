#!/usr/bin/env python3
import fileinput

score=0
for line in fileinput.input():
    op,res = line.strip().split()
    op="ABC".index(op)
    res="XYZ".index(res)-1
    me=(op+res)%3

    if res == -1:# i lose
        pass
    elif res == 0:# tie
        score += 3
    else:
        score += 6
    score += (1+me)
print(score)
