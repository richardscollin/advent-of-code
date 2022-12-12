#!/usr/bin/env python3
from collections import defaultdict, deque
lines = open(0).read().splitlines()
#print(lines)

starts = []
grid = []
for _ in range(len(lines)):
    grid.append([0 for _ in range(len(lines[0]))])

def print_graph():
    for r in range(len(lines)):
        print(graph[r])

for r in range(len(lines)):
    for c in range(len(lines[0])):
        v = lines[r][c]

        if v == "S" or v == 'a':
            starts.append((r,c))
            height = 0
        elif v == "E":
            end = (r,c)
            height = ord('z') - ord('a')
        else:
            height = ord(v) - ord('a')
        grid[r][c] = height

#print_graph()

def grid2graph(grid):
    adj_list = defaultdict(list)
    H = len(grid)
    W = len(grid[0])
    for r in range(H):
        for c in range(W):
            for dr, dc in [(r,c-1), (r,c+1), (r-1,c), (r+1,c)]: # l, r, u, d
                #print("dr dc", dr, dc)
                if 0 <= dr < H and 0 <= dc < W: # in boundry
                    if grid[dr][dc] <= grid[r][c] + 1: # can step
                        adj_list[(r,c)].append((dr,dc))

    return adj_list


graph = grid2graph(grid)
#print(graph)
#print(graph[(0,0)])

def bfs(graph, starts, end):
    q = deque()
    visited = set()
    for start in starts:
        q.append(start)
        visited.add(start)

    dist = defaultdict(lambda: float('inf'))
    pred = defaultdict(lambda: -1)

    while q:
        v = q.popleft()
        if v == end: break
        neighbors = graph[v]
        for neighbor in neighbors:
            if neighbor not in visited:
                visited.add(neighbor)
                dist[neighbor] = dist[v] + 1
                pred[neighbor] = v
                q.append(neighbor)

    result = []
    curr = end
    while pred[curr] != -1:
        curr = pred[curr]
        result.append(curr)
    #print(end)
    #print(result)
    print(len(result))

bfs(graph, starts, end)
