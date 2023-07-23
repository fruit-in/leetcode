# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTilt(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            l_sum, l_tilt = helper(root.left)
            r_sum, r_tilt = helper(root.right)

            return (root.val + l_sum + r_sum, l_tilt + r_tilt + abs(l_sum - r_sum))

        return helper(root)[1]
