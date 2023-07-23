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
        nset = set()
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if not cur.children or cur.children[0] in nset:
                vals.append(cur.val)
                nset.add(cur)
            else:
                nodes.append(cur)
                for child in cur.children[::-1]:
                    nodes.append(child)
        return vals
