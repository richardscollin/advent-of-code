#!/usr/bin/env python3
import math
lines = open(0).read().splitlines()

visited = set()
tails = [0j]*9
head = 0j

direction_map = {
 "U": 1j,
 "D": -1j,
 "R": 1+0j,
 "L": -1+0j,
}

def print_grid():
    for r in range(4,-1,-1):
        for c in range(6):
            if (c,r) == head:
                print("H",end="")
            elif (c,r) in tails:
                i=tails.index((c,r))+1
                print(i,end="")
            elif (c,r) in visited:
                print("#",end="")
            else: print(".",end="")
        print()
    print("\n")

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
    dist = abs(tail-head)
    #print(dist)
    #print(tail)
    #print(head)
    if dist <= 2**.5: return tail

    dx = step_dir(head.real,tail.real)
    dy = step_dir(head.imag,tail.imag)
    return tail + complex(dx,dy)

print_grid()
for l in lines:
    direction, steps = l.split()
    direction = direction_map[direction]
    steps = int(steps)
    while steps > 0:
        head += direction
        lead = head
        for i, t in enumerate(tails):
            tails[i] = follow_head(t, lead)
            lead = tails[i]
            if i == 8:
                visited.add(tails[i])
        steps -= 1
        #print_grid()


#print(f"{head} {tail}")
#print(visited)
print(len(visited))

