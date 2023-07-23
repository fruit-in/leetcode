"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if not root:
            return []
        ret = []
        nodes = [root]
        while nodes:
            level = []
            temp = []
            for cur in nodes:
                level.append(cur.val)
                temp.extend(cur.children)
            nodes = temp
            ret.append(level)
        return ret
