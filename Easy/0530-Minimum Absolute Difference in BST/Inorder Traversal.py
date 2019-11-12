# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getMinimumDifference(self, root: TreeNode) -> int:
        nodes = []
        curr = root
        prev = float("-inf")
        min_diff = float("+inf")

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            min_diff = min(min_diff, curr.val - prev)
            prev = curr.val

            curr = curr.right

        return min_diff
