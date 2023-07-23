"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if cur:
                vals.append(cur.val)
                for n in cur.children[::-1]:
                    nodes.append(n)
        return vals
