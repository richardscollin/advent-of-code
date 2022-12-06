#!/usr/bin/env python3
import fileinput

score=0
for line in fileinput.input():
    op,me = line.strip().split()
    op="ABC".index(op)
    me="XYZ".index(me)

    if (op + 1) % 3 == me:# i win
        score += 6
    elif (me + 1) % 3 == op:# u win
        pass
    else:
        score += 3
    score += (1+me)
print(score)
