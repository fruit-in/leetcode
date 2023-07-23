"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        vals = []
        for n in root.children:
            vals.extend(self.postorder(n))
        vals.append(root.val)
        return vals
