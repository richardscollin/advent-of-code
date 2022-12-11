#!/usr/bin/env python3
"""
Monkey 0:
  Starting items: 63, 57
  Operation: new = old * 11
  Test: divisible by 7
    If true: throw to monkey 6
    If false: throw to monkey 2
"""

monkey_div = 1

class Monkey:
    def __init__(self,idx,items,op,divisor,d_true,d_false):
        self.idx = idx
        self.items = items
        self.op = op
        self.divisor = divisor
        self.d_true = d_true
        self.d_false = d_false
        self.inspection_count = 0

    def throw(self, monkeys):
        global monkey_div
        self.inspection_count += len(self.items)

        for item in self.items:
            new_level = self.op(item) % monkey_div
            send_to = self.d_true if new_level % self.divisor == 0 else self.d_false
            monkeys[send_to].items.append(new_level)

        self.items = []

monkeys = []

lines = iter(open(0))
last_int = lambda: int(next(lines).split()[-1])
while True:
    idx   = int((next(lines)[:-2]).split()[1])
    items = list(map(int,next(lines).split(":")[1].split(", ")))
    op    = eval(f"lambda old: {next(lines)[19:]}") # stolen from u/4HbQ
    monkeys.append(Monkey(idx, items, op, last_int(), last_int(), last_int()))
    if next(lines, None) == None: break

# See Chinese Remainder Theorem
for m in monkeys:
    monkey_div *= m.divisor

for i in range(10000):
    for m in monkeys:
        m.throw(monkeys)

b = sorted(m.inspection_count for m in monkeys)[-2:]
print(b[0]*b[1])
