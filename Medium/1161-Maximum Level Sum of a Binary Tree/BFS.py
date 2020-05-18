# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxLevelSum(self, root: TreeNode) -> int:
        curr_level = [root]
        max_sum = root.val
        x = 1
        ret = 1

        while curr_level:
            curr_sum = sum(n.val for n in curr_level)
            if curr_sum > max_sum:
                max_sum = curr_sum
                ret = x

            curr_level = [c for n in curr_level for c in [n.left, n.right] if c]
            x += 1

        return ret
