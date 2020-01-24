# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isValidBST(self, root: TreeNode) -> bool:
        nodes = []
        curr = root
        prev_val = float('-inf')

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            if prev_val >= curr.val:
                return False
            prev_val = curr.val

            curr = curr.right

        return True
