# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def longestZigZag(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[0]

    def dfs(self, root: Optional[TreeNode]) -> (int, int, int):
        if root is None:
            return (0, -1, -1)

        left = self.dfs(root.left)
        right = self.dfs(root.right)
        maxpath = max(left[0], left[1], left[2] + 1,
                      right[0], right[1] + 1, right[2])

        return (maxpath, left[2] + 1, right[1] + 1)
