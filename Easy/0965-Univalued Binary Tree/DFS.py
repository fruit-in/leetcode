# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isUnivalTree(self, root: TreeNode) -> bool:
        left, right = False, False
        if not root.left:
            left = True
        elif root.val == root.left.val:
            left = self.isUnivalTree(root.left)
        if not root.right:
            right = True
        elif root.val == root.right.val:
            right = self.isUnivalTree(root.right)
        return left and right
