# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def kthLargestLevelSum(self, root: Optional[TreeNode], k: int) -> int:
        def dfs(root: Optional[TreeNode], depth: int) -> None:
            if root is not None:
                if len(levelsum) < depth:
                    levelsum.append(0)
                levelsum[depth - 1] += root.val
                dfs(root.left, depth + 1)
                dfs(root.right, depth + 1)

        levelsum = []
        dfs(root, 1)

        return sorted(levelsum)[-k] if k <= len(levelsum) else -1
