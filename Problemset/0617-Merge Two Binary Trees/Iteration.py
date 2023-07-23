# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def mergeTrees(self, t1: TreeNode, t2: TreeNode) -> TreeNode:
        if not t1:
            return t2
        if not t2:
            return t1
        stack = [(t1, t2)]
        while stack:
            n1, n2 = stack.pop()
            n1.val += n2.val
            if n1.left and n2.left:
                stack.append((n1.left, n2.left))
            elif not n1.left and n2.left:
                n1.left = n2.left
            if n1.right and n2.right:
                stack.append((n1.right, n2.right))
            elif not n1.right and n2.right:
                n1.right = n2.right
        return t1
