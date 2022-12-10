#!/usr/bin/env python3
t,s,X=0,0,1
for cmd, *v in (l.split() for l in open(0)):
 for j in range(2):
  i = t % 40; t += 1
  print([".", "#"][X-1 <= i <= X+1], end=["","\n"][i == 39])
  if (t-20) % 40 == 0: s += t * X
  if cmd == "noop": break
  if j == 1: X += int(v[0])
print(s)
