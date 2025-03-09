# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def dfs(root: Optional[TreeNode]) -> (int, int):
            if root is None:
                return (-1000, 0)

            lsubmax, lchildmax = dfs(root.left)
            rsubmax, rchildmax = dfs(root.right)
            submax = max(lsubmax, rsubmax, lchildmax + rchildmax + root.val)
            rootmax = max(0, root.val + max(lchildmax, rchildmax))

            return (submax, rootmax)

        return dfs(root)[0]
