# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def smallestFromLeaf(self, root: Optional[TreeNode]) -> str:
        return self.dfs("", root)

    def dfs(self, s: str, root: Optional[TreeNode]) -> str:
        if root is None:
            return s

        s = chr(root.val + 97) + s

        if root.left is None:
            return self.dfs(s, root.right)
        elif root.right is None:
            return self.dfs(s, root.left)
        else:
            return min(self.dfs(s, root.left), self.dfs(s, root.right))
