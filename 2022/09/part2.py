#!/usr/bin/env python3
sign = lambda x: (x > 0) - (x < 0)

def follow(follower, leader):
    mid = leader - follower
    if abs(mid) <= abs(1+1j): return follower
    return follower + complex(sign(mid.real), sign(mid.imag))

part1, part2 = set(), set()
head, *tail = [0j]*10
directions = dict(U=1j, D=-1j, R=1+0j, L=-1+0j)

for line in open(0):
    direction, steps = line.split()
    for _ in range(int(steps)):
        head += directions[direction]
        lead = head
        for i, t in enumerate(tail):
            tail[i] = follow(t, lead)
            lead = tail[i]
            if i == 0: part1.add(tail[i])
            elif i == len(tail)-1: part2.add(tail[i])

print(len(part1), len(part2))
