# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def maxDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        lmax = self.maxDepth(root.left)
        rmax = self.maxDepth(root.right)
        return max(lmax, rmax) + 1
