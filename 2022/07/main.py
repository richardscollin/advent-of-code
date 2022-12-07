#!/usr/bin/env python3
from collections import defaultdict
import fileinput

class TreeNode:
    size = 0

    def __init__(self):
        self.children = defaultdict(TreeNode)

    def add_file(self, name, size):
        self.children[name].size = size

    def get_dir(self,folder):
        return self.children[folder]
    
    def stat(self):
        return self.size + sum(c.stat() for c in self.children.values())

    def all_dirs(self, limit=100000):
        a = []
        if self.size == 0 and self.stat() <= limit: # is dir
            a+=[self]

        for k,d in self.children.items():
            a+=d.all_dirs(limit=limit)

        return a

stack = [TreeNode()]
for line in fileinput.input():
    line = line.strip()
    if line.startswith("$"):
        _, cmd, *tail = line.split(" ")
        if cmd == "cd":
            target = tail[0]
            if target == "..": stack.pop()
            else: stack.append(stack[-1].get_dir(target))
    else:
        size, name = line.split(" ")
        if size != "dir":
            stack[-1].add_file(name, int(size))

print(sum(d.stat() for d in stack[0].all_dirs()))
