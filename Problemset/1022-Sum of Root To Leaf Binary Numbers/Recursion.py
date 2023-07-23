# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        def helper(root: TreeNode, n: int) -> int:
            n = (n << 1) + root.val

            if root.left and root.right:
                return helper(root.left, n) + helper(root.right, n)
            elif root.left:
                return helper(root.left, n)
            elif root.right:
                return helper(root.right, n)
            else:
                return n

        return helper(root, 0)
