#!/usr/bin/env python3.11
def irange(*a):
    a,b = sorted(a)
    return range(a,b+1)

sensors = set()
beacons = set()
grid = set()
for l in open(0):
    words = l.strip().split()
    x1=int(words[2].split('=')[1][:-1])
    y1=int(words[3].split('=')[1][:-1])
    x2=int(words[8].split('=')[1][:-1])
    y2=int(words[9].split('=')[1])
    man_dist = abs(x1-x2)+abs(y1-y2)

    grid.add((x1,y1))
    grid.add((x2,y2))

    sensors.add((x1,y1))
    beacons.add((x2,y2))

    for i in range(man_dist + 1):
        j = man_dist - i
        for dx in irange(-i, i):
            for dy in irange(-j, j):
                grid.add((x1+dx,y1+dy))


def count(grid, beacons, y):
    min_x = min(p[0] for p in beacons)
    max_x = max(p[0] for p in beacons)

    count = 0
    for x in range(min_x, max_x+1):
        if (x, y) in grid and (x,y) not in beacons:
            count += 1
    return count

def print_grid(grid, sensors, beacons):
    min_y = min(p[1] for p in grid)
    max_y = max(p[1] for p in grid)
    min_x = min(p[0] for p in grid)
    max_x = max(p[0] for p in grid)

    for y in range(min_y, max_y+1):
        print(f"{y:02}:", end="")
        for x in range(min_x, max_x+1):
            ch = " "
            if (x,y) in sensors:
                ch = 'S'
            elif (x,y) in beacons:
                ch = 'B'
            elif (x,y) in grid:
                ch = "."
            print(ch, end="")
        print()

print_grid(grid, sensors, beacons)
print(count(grid, beacons, 10))
