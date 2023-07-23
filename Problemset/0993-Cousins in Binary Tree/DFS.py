# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        parent, depth = {root.val: None}, {}
        def dfs(root: TreeNode, dep: int):
            depth[root.val] = dep
            if root.left:
                parent[root.left.val] = root
                dfs(root.left, dep + 1)
            if root.right:
                parent[root.right.val] = root
                dfs(root.right, dep + 1)

        dfs(root, 0)
        return depth[x] == depth[y] and parent[x] != parent[y]
