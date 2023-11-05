# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxSumBST(self, root: Optional[TreeNode]) -> int:
        def dfs(root: Optional[TreeNode]) -> (bool, int, int, int, int):
            if root is None:
                return (True, 40001, -40001, 0, 0)

            isbstl, minl, maxl, suml, retl = dfs(root.left)
            isbstr, minr, maxr, sumr, retr = dfs(root.right)
            isbstt = isbstl and isbstr and root.val > maxl and root.val < minr

            if isbstt:
                sumt = suml + sumr + root.val
                return (True, min(minl, root.val), max(maxr, root.val), sumt, max(sumt, retl, retr))
            else:
                return (False, 0, 0, 0, max(retl, retr))

        return dfs(root)[4]
