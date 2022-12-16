#!/usr/bin/env python3.11
import sys
from interval_tree import insert, inOrder, merge
# this one is messy, I won't bother cleaning it up
# solving involved running a section of code comenting it out
# and rerunning with the produced value

#y = 2655411
# 5968076 is too high
# 7257164 is too high
readings = []
for line in open(0):
    words = line.strip().split()
    sx=int(words[2].split('=')[1][:-1])
    sy=int(words[3].split('=')[1][:-1])
    bx=int(words[8].split('=')[1][:-1])
    by=int(words[9].split('=')[1])
    dist = abs(sx-bx)+abs(sy-by)
    readings.append(((sx,sy),(bx,by), dist))

#for y in range(4000000):
ranges = [range(1000000), range(1000000, 2000000), range(2000000, 3000000), range(3000000, 4000001)]

# find y
for y in ranges[int(sys.argv[1])]:
    x_root = None

    for (sx,sy),(bx,by), dist in readings:
        if sy - dist <= y <= sy + dist:
            to = dist - abs(sy-y)
            x_root = insert(x_root, sx - to, sx + to)

    x = merge(x_root, [])
    size = sum(d[1]-d[0]+1 for d in x)
    if size != 4000001 and size != 0:
        print(y, size)


# y is 2655410
# now find x
#y = 2655410
#for x in ranges[int(sys.argv[1])]:
#    x_root = None
#
#    for (sx,sy),(bx,by), dist in readings:
#        if sx - dist <= x <= sx + dist and sy-dist <= y <= sy+dist:
#            to = dist - abs(sx-x)
#            x_root = insert(x_root, sy - to, sy + to)
#
#    z = merge(x_root, [])
#    size = sum(d[1]-d[0]+1 for d in z)
#    if size != 4000001 and size != 0:
#        print(x, size)
#        #print(z)
#
