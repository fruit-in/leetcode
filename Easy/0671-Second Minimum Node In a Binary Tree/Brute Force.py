# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        vals = set()
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            vals.add(curr.val)
            if curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        vals.remove(min(vals))

        return min(vals) if vals else -1
