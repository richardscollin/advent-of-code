#!/usr/bin/env python3
"""
cleaned up part 1 and 2
accepts input from stdin
"""
from collections import defaultdict, deque
from itertools import product
lines = open(0).read().splitlines()
H, W = len(lines), len(lines[0])

part1 = None
part2 = []
grid = [[0] * W for _ in range(H)]
for r, c in product(range(H), range(W)):
    v = lines[r][c]

    if v == "S":
        part1 = (r,c)
        v = 'a'
    elif v == "E":
        end = (r,c)
        v = 'z'

    height = ord(v) - ord('a')
    if height == 0:
        part2.append((r,c))

    grid[r][c] = height

def climbable(grid, r, c):
    for dr, dc in [(r,c-1), (r,c+1), (r-1,c), (r+1,c)]: # l, r, u, d
        if 0 <= dr < H and \
           0 <= dc < W and \
           grid[dr][dc] <= grid[r][c] + 1:
            yield (dr,dc)

def shortest_path_length(grid, starts, end):
    q = deque(starts)
    visited = set(starts)
    dist = defaultdict(int)

    v, dist[end] = None, -1
    while q and v != end:
        v = q.popleft()
        for neighbor in climbable(grid, *v):
            if neighbor not in visited:
                visited.add(neighbor)
                q.append(neighbor)
                dist[neighbor] = dist[v] + 1

    return dist[end]

print(shortest_path_length(grid, [part1], end))
print(shortest_path_length(grid, part2, end))
