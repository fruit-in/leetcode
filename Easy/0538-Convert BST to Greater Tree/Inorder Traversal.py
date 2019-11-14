# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def convertBST(self, root: TreeNode) -> TreeNode:
        nodes = []
        curr = root
        sum = 0

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
