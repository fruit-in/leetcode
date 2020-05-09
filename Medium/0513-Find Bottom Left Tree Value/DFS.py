# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findBottomLeftValue(self, root: TreeNode) -> int:
        def dfs(root: TreeNode, depth: int) -> (int, int):
            ori_depth = depth
            val, depth = root.val, depth

            if root.left:
                val, depth = dfs(root.left, ori_depth + 1)
            if root.right:
                right_val, right_depth = dfs(root.right, ori_depth + 1)
                if right_depth > depth:
                    val, depth = right_val, right_depth

            return val, depth

        return dfs(root, 1)[0]
