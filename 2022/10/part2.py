#!/usr/bin/env python3
cycle = 0
X = 1
signal = 0

def end_of_cycle():
    global cycle, signal
    i = cycle % 40

    if i == 0 and cycle:
        print()

    if X-1 <= i <= X+1:
        print("#",end="")
    else:
        print(".",end="")

    cycle += 1

    if (cycle-20) % 40 == 0:
        signal += cycle * X

for line in open(0):
    cmd, *v = line.split()
    end_of_cycle()

    if cmd == "addx":
        end_of_cycle()
        X += int(v[0])
print()

#print(signal)
