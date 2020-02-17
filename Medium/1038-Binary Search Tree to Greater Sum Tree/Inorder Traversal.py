# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def bstToGst(self, root: TreeNode) -> TreeNode:
        stack = []
        curr = root
        sum = 0

        while stack or curr:
            while curr:
                stack.append(curr)
                curr = curr.right

            curr = stack.pop()
            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
