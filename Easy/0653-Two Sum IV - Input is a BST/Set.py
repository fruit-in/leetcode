# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        def helper(root: TreeNode, k: int, vals: set) -> bool:
            if not root:
                return False

            if k - root.val in vals:
                return True

            vals.add(root.val)

            return helper(root.left, k, vals) or helper(root.right, k, vals)

        return helper(root, k, set())
