ids = {
    int(x.translate("".maketrans("FBLR", "0101")), 2)
    for x in [line.strip() for line in open("input.txt").readlines()]
}
# print(max(ids))
print(range(min(ids), max(ids) + 1))
# print(sum() - sum(ids))
