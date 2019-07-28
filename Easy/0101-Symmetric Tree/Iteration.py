# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSymmetric(self, root: TreeNode) -> bool:
        if not root:
            return True
        stack = [root.left, root.right]
        while stack:
            n1, n2 = stack.pop(), stack.pop()
            if not n1 and not n2:
                continue
            if not n1 or not n2 or n1.val != n2.val:
                return False
            stack.append(n1.left)
            stack.append(n2.right)
            stack.append(n1.right)
            stack.append(n2.left)
        return True
