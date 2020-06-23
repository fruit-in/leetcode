# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def generateTrees(self, n: int) -> List[TreeNode]:
        def helper(m: int, n: int) -> List[TreeNode]:
            if m > n:
                return [None]

            ret = []

            for val in range(m, n + 1):
                for left in helper(m, val - 1):
                    for right in helper(val + 1, n):
                        ret.append(TreeNode(val, left, right))

            return ret

        return helper(1, n) if n > 0 else []
