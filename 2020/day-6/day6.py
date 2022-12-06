import string
groups = [group for group in open("input.txt").read().split("\n\n")]

# part 1
# acc = 0
# for group in groups:
#     acc += len(set(group.replace('\n', '')))
# print(acc)


# part 2
acc = 0
for group in groups:
    selected = set(string.ascii_lowercase)

    for person in group.splitlines():
        selected.intersection_update(person)
    acc += len(selected)

print(acc)
