import re

lines = [line.strip() for line in open("input.txt").readlines()]

m = {}
for line in lines:
    triple = re.split(r" bags contain (\d+) | bags?, (\d+) | bags?.", line)
    triple.pop()

    triple = list(filter(None, triple))

    if len(triple) % 2 == 1:
        bags = []
        for (n, c) in zip(triple[1::2], triple[2::2]):
            bags.append({"color": c, "amount": int(n)})
        m[triple[0]] = bags
    else:
        m[triple[0]] = []


def dfs(m, bag, target):
    for option in m[bag]:
        if target == option["color"]:
            return True

    for option in m[bag]:
        c = option["color"]
        if dfs(m, c, target):
            return True

    return False

# TODO we could memoize this


def numbags(m, color):
    if len(m[color]) == 0:
        return 0
    else:
        acc = 0
        for bag in m[color]:
            acc += bag["amount"] * (1 + numbags(m, bag["color"]))
        return acc


acc = 0
for c in m.keys():
    contains = dfs(m, c, "shiny gold")
    if contains:
        acc += 1

print(acc)
print(numbags(m, "shiny gold"))
