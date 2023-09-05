# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def distributeCoins(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[2]

    def dfs(self, root: Optional[TreeNode]) -> (int, int, int):
        if root is None:
            return (0, 0, 0)

        lnodes, lvals, lmoves = self.dfs(root.left)
        rnodes, rvals, rmoves = self.dfs(root.right)

        return (lnodes + rnodes + 1,
                lvals + rvals + root.val,
                lmoves + rmoves + abs(lnodes - lvals) + abs(rnodes - rvals))
