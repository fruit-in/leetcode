# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        if not root:
            return 0

        nodes = [(root, False)]
        s = 0

        while nodes:
            cur, left = nodes.pop()
            if cur.left:
                nodes.append((cur.left, True))
            if cur.right:
                nodes.append((cur.right, False))
            if left and not cur.left and not cur.right:
                s += cur.val

        return s
