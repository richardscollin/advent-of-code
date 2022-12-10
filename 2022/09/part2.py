#!/usr/bin/env python3
sign = lambda x: (x > 0) - (x < 0)

part1, part2 = set(), set()
head, *tail = [0j]*10
directions = dict(U=1j, D=-1j, R=1+0j, L=-1+0j)

for line in open(0):
    direction, steps = line.split()
    for _ in range(int(steps)):
        head += directions[direction]
        leader = head

        for i, follower in enumerate(tail):
            mid = leader - follower
            diff = complex(sign(mid.real), sign(mid.imag))
            tail[i] += diff if abs(mid) >= 2 else 0
            leader = tail[i]

        part1.add(tail[0])
        part2.add(tail[-1])

print(len(part1), len(part2))
