#!/usr/bin/env python3
import fileinput

def pair_within(o,i):
    return o[0]<=i[0] and i[1] <= o[1]


c=0
for line in fileinput.input():
    s1,e1,s2,e2 = map(int,line.replace("-",",").split(","))
    p1=(s1,e1)
    p2=(s2,e2)
    if pair_within(p1,p2) or pair_within(p2,p1):
        c+=1

print(c) 
