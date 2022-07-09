# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def averageOfSubtree(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[2]

    def dfs(self, root: TreeNode) -> (int, int, int):
        if not root:
            return (0, 0, 0)

        count_left, sum_left, ret_left = self.dfs(root.left)
        count_right, sum_right, ret_right = self.dfs(root.right)
        count_root = count_left + count_right + 1
        sum_root = sum_left + sum_right + root.val
        ret_root = ret_left + ret_right + \
            (1 if root.val == sum_root // count_root else 0)

        return (count_root, sum_root, ret_root)
