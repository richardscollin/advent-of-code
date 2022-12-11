#!/usr/bin/env python3
from functools import partial
from operator import mul, add
"""
Monkey 0:
  Starting items: 63, 57
  Operation: new = old * 11
  Test: divisible by 7
    If true: throw to monkey 6
    If false: throw to monkey 2
"""

class Monkey:
    idx = None
    items = []
    op = None
    divisor = None
    d_true = None
    d_false = None
    inspection_count = 0

    def __init__(self,idx,items,op,divisor,d_true,d_false):
        self.idx = idx
        self.items = items
        self.op = op
        self.divisor = divisor
        self.d_true = d_true
        self.d_false = d_false

    def throw(self, monkeys):
        self.inspection_count += len(self.items)

        keep = []
        for item in self.items:
            new_level = self.op(item) // 3

            if new_level % self.divisor == 0:
                send_to = self.d_true
            else:
                send_to = self.d_false

            if send_to != self.idx:
                monkeys[send_to].items.append(new_level)
            else:
                keep.append(new_level)

        self.items = keep

monkeys = []

lines = iter(open(0))
while lines:
    idx = int((next(lines)[:-2]).split()[1])
    items = list(map(int,next(lines).split(":")[1].split(", ")))
    operation = next(lines).split()

    muladd = operation[4]
    if operation[5] == "old":
        if muladd == "*":
            op = lambda x: x**2
        else:
            op = lambda x: x*2
    else:
        operand = int(operation[5])
        if muladd == "*":
            op = partial(mul, operand)
        else:
            op = partial(add, operand)

    last_int = lambda: int(next(lines).split()[-1])
    divisor, d_true, d_false = last_int(), last_int(), last_int()

    monkeys.append(Monkey(idx, items, op, divisor, d_true, d_false))
    if next(lines, None) == None: break

for _ in range(20):
    for m in monkeys:
        m.throw(monkeys)

b = sorted(m.inspection_count for m in monkeys)[-2:]
print(b[0]*b[1])
