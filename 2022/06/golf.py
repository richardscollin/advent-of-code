#!/usr/bin/env python3
s = input()
n = 14
print(next(i for i in range(n, len(s)) if len(set(s[i-n:i])) == n))
