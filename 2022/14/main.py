#!/usr/bin/env python3.11
def irange(a, b):
    return range(min(a,b), max(a,b)+1)

grid = set()
for line in open(0):
    #points = [tuple(map(int, p.split(","))) for p in pairs]
    points = [eval(p) for p in line.split(" -> ")]
    for (x1, y1), (x2, y2) in zip(points[::], points[1::]):
        if x1 == x2:
            for y in irange(y1, y2):
                grid.add((x1, y))
        elif y1 == y2:
            for x in irange(x1, x2):
                grid.add((x, y1))
        else:
            raise Error("Unexpected input")

def fall1(grid, x, y):
    down       =   (x, y+1)
    down_left  = (x-1, y+1)
    down_right = (x+1, y+1)
    for direction in [down, down_left, down_right]:
        if direction not in grid:
            return direction
    return (x, y)

def fall(grid, max_y, loc):
    prev = None
    while loc != prev:
        prev, loc = loc, fall1(grid, *loc)
        if loc[1] > max_y:
            return None
    return loc

def corners(grid):
    max_x = max(loc[0] for loc in grid)
    min_x = min(loc[0] for loc in grid)
    max_y = max(loc[1] for loc in grid)
    return (min_x, max_x, 0, max_y)

def print_grid(grid):
    min_x, max_x, min_y, max_y = corners(grid)

    for y in range(min_y, max_y+1):
        for x in range(min_x, max_x+1):
            ch = "."
            if (x,y) in grid:
                ch = "#"
            print(ch, end="")
        print()

sand_count = 0
sand_start = (500, 0)
print_grid(grid)
_, _, _, max_y = corners(grid)
while rest := fall(grid, max_y, sand_start):
    grid.add(rest)
    sand_count += 1


print(sand_count)
