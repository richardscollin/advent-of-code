#!/usr/bin/env python3
cycle = 0
X = 1
signal = 0

def end_of_cycle():
    global cycle, signal
    cycle += 1

    if (cycle-20) % 40 == 0:
        signal += cycle * X

for line in open(0):
    cmd, *v = line.split()
    end_of_cycle()

    if cmd == "addx":
        end_of_cycle()
        X += int(v[0])

print(signal)
