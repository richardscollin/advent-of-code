#!/usr/bin/env python3
import fileinput
import numpy as np


def main(nums):
    nums.sort()
    target = 2020

    for e1 in nums:
        for e2 in nums:
            x = target - (e1 + e2)
            if x in nums:
                print(x * e1 * e2)
                break


if __name__ == "__main__":
    main([int(line.strip()) for line in fileinput.input('input.txt')])
