#!/usr/bin/env python3
from collections import deque
import fileinput


stacks=[]
instr=False
first=True
for line in fileinput.input():
    if line == "\n":
        instr=True
        continue

    if first:
        first=False
        n=(len(line)-3)//4+1
        for i in range(n):
            stacks.append(deque())

    if instr:
        # move 1 from 2 to 1
        l=line.split(" ")
        count,f,t=int(l[1]),int(l[3]),int(l[5])
        r=reversed(list(stacks[f-1].pop() for _ in range(count)))
        stacks[t-1].extend(r)

    else: # build grid
        chars = line[1:len(line):4]
        if chars.isnumeric():continue

        for i,c in enumerate(chars):
            if c == " ": continue
            stacks[i].appendleft(c)

print("".join(q[-1] for q in stacks))
