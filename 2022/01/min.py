#!/usr/bin/env python3
l=sorted(sum(map(int,e.split())) for e in open(0).read().split("\n\n"))
print(l[-1])
print(sum(l[:-3]))
