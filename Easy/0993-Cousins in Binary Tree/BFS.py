# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        nodes = [root]
        while nodes:
            nodes = [node.left for node in nodes if node] \
                + [node.right for node in nodes if node]
            vals = [node.val if node else 0 for node in nodes]
            if x in vals and y in vals:
                return abs(vals.index(x) - vals.index(y)) != len(vals) / 2
            elif x in vals or y in vals:
                return False
