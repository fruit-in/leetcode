# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumEvenGrandparent(self, root: TreeNode) -> int:
        stack = [root]
        ret = 0

        while stack:
            curr = stack.pop()

            if curr.left:
                stack.append(curr.left)
                if curr.val % 2 == 0:
                    ret += curr.left.left.val if curr.left.left else 0
                    ret += curr.left.right.val if curr.left.right else 0
            if curr.right:
                stack.append(curr.right)
                if curr.val % 2 == 0:
                    ret += curr.right.left.val if curr.right.left else 0
                    ret += curr.right.right.val if curr.right.right else 0

        return ret
