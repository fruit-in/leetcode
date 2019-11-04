# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        def helper(root: TreeNode, path_sum: dict, prev_sum: int, sum: int) -> int:
            if not root:
                return 0

            curr_sum = prev_sum + root.val
            ret = 0

            if path_sum.get(curr_sum - sum):
                ret += path_sum.get(curr_sum - sum)

            if not path_sum.get(curr_sum):
                path_sum[curr_sum] = 0
            path_sum[curr_sum] += 1

            ret += helper(root.left, dict(path_sum), curr_sum, sum)
            ret += helper(root.right, dict(path_sum), curr_sum, sum)

            return ret


        return helper(root, {0: 1}, 0, sum)
