# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        if not root:
            return 0

        s = 0
        if root.left and not root.left.left and not root.left.right:
            s += root.left.val
        elif root.left:
            s += self.sumOfLeftLeaves(root.left)
        if root.right:
            s += self.sumOfLeftLeaves(root.right)

        return s
