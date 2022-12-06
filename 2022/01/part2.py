#!/usr/bin/env python3
import fileinput

elves=[]
calories=0
for line in fileinput.input():
    if line != '\n':
        calories += int(line)
    else:
        elves+=[calories]
        calories=0
elves+=[calories]
print(sum(sorted(elves)[-3:]))
