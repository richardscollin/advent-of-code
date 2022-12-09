#!/usr/bin/env python3
import math
lines = open(0).read().splitlines()

visited = set()
tail = (0,0)
head = (0,0)

direction_map = {
 "U": (0,1),
 "D": (0,-1),
 "R": (1,0),
 "L": (-1,0),
}

def print_grid():
    for r in range(4,-1,-1):
        for c in range(6):
            if (c,r) == head:
                print("H",end="")
            elif (c,r) == tail:
                print("T",end="")
            elif (c,r) in visited:
                print("#",end="")
            else: print(".",end="")
        print()
    print("\n")

def add_points(p1,p2):
    return (p1[0]+p2[0],p1[1]+p2[1])

def step_dir(head_c, tail_c): # tail_coordinate
    c=(head_c - tail_c)/2
    if c.is_integer():
        return int(c)
    # round towards head
    if head_c > tail_c:
        c += .5
    else:
        c -= .5
    return round(c)

def follow_head(tail, head):
    """returns the new location of tail"""
    dist = ((tail[0]-head[0])**2 + (tail[1]-head[1])**2)**.5
    #print(dist)
    #print(tail)
    #print(head)
    if dist <= 2**.5: return tail

    dx = step_dir(head[0],tail[0])
    dy = step_dir(head[1],tail[1])
    return add_points(tail, (dx,dy))

#print_grid()
for l in lines:
    direction, steps = l.split()
    direction = direction_map[direction]
    steps = int(steps)
    while steps > 0:
        head = add_points(head, direction)
        tail = follow_head(tail, head)
        visited.add(tail)
        steps -= 1
        #print_grid()


#print(f"{head} {tail}")
#print(visited)
print(len(visited))

