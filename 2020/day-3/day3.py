#!/usr/bin/env python3
import math


def main(lines, slope):
    # So we want an expanded version of the string where
    # each row is the full width. We'll then iterate through
    # and count the number of trees. This naive implementation
    # is memory intensive.

    c = 0
    r = 0

    trees = 0
    for r in range(len(lines)):
        if (lines[r][c] == "#"):
            trees += 1
        c += slope
    return trees


def main2(lines):
    c = 0
    r = 0
    trees = 0
    for r in range(0, len(lines), 2):
        if (lines[r][c] == "#"):
            trees += 1
        c += 1
    return trees


if __name__ == "__main__":
    lines = []
    with open("input") as f:
        lines = f.readlines()

    height = len(lines)
    width = len(lines[0])
    steps = 7 / 1
    # width * X = height * steps
    multiplier = 8 + math.ceil(height * steps / float(width))
    print(f"height: {height}\n"
          + f"width: {width}\n"
          + f"mult: {multiplier}\n"
          + f"exp: {multiplier * width}\n")

    trimmed = [line.strip() for line in lines]
    expanded = [line * multiplier for line in trimmed]
    slopes = [1, 3, 5, 7]

    acc = 1
    for s in slopes:
        acc *= main(expanded, s)

    acc *= main2(expanded)
    print(acc)
