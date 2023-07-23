# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def minDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        if not root.left and not root.right:
            return 1
        lmin, rmin = 0, 0
        if root.left:
            lmin = self.minDepth(root.left) + 1
        if root.right:
            rmin = self.minDepth(root.right) + 1
        if lmin and rmin:
            return min(lmin, rmin)
        else:
            return lmin if lmin else rmin
