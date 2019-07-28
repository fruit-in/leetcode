"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0
        depth = 1
        nodes = [n for n in root.children]
        while nodes:
            depth += 1
            nodes = [n for node in nodes for n in node.children]
        return depth
