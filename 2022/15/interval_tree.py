class IntervalTreeNode:
    def __init__(self, low, high, min_low=0, max_high=4000000):
        self.low = max(low, min_low)
        self.high = min(high, max_high)
        self.max = self.high
        self.left = None
        self.right = None

    def stat(self):
        left = self.left.stat() if self.left else 0
        right = self.right.stat() if self.right else 0
        return self.high - self.low + 1 + left + right

    def __str__(self):
        return f"({self.low},{self.high})"

def insert(node, low, high):
    if node == None:
        return IntervalTreeNode(low, high)

    if low <= node.low:
        node.left = insert(node.left, low, high)
    else:
        node.right = insert(node.right, low, high)

    node.max = max(node.max, min(high, 4000000))
    return node

def merge(node, data):
    if node == None:
        return data

    merge(node.left, data)
    if data and node.low <= data[-1][1] + 1:
        data[-1] = (data[-1][0], max(node.high, data[-1][1]))
    else:
        data.append((node.low, node.high))
    merge(node.right, data)
    return data


def inOrder(node):
    if node == None:
        return

    inOrder(node.left)
    print(node, end="")
    inOrder(node.right)
