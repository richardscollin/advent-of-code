#!/usr/bin/env python3
import fileinput
from pprint import pprint


def main(lines):
    processed = [process(line) for line in lines]
    pprint(processed)
    print(sum([p['passed'] for p in processed]))


def process(line):
    (pattern, password) = line.split(": ")
    (length, letter) = pattern.split(" ")
    (left_index, right_index) = length.split("-")

    left_index = int(left_index) - 1
    right_index = int(right_index) - 1
    count = password.count(letter)

    passed = (password[left_index] == letter) ^ (
        password[right_index] == letter)

    return {
        "password": password,
        "letter": letter,
        "left_index": left_index,
        "right_index": right_index,
        "count": count,
        "passed": passed,
    }


if __name__ == "__main__":
    lines = [line.strip() for line in fileinput.input('input.txt')]
    main(lines)
