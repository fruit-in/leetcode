# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def maxAncestorDiff(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int, int):
            if not root:
                return (100001, -1, -1)

            l_min, l_max, l_diff = helper(root.left)
            r_min, r_max, r_diff = helper(root.right)

            lr_min = min(l_min, r_min)
            lr_max = max(l_max, r_max)

            diff = max(l_diff, r_diff)
            if lr_min != 100001:
                diff = max(diff, abs(root.val - lr_min))
            if lr_max != -1:
                diff = max(diff, abs(root.val - lr_max))

            return (min(root.val, lr_min), max(root.val, lr_max), diff)

        return helper(root)[2]
