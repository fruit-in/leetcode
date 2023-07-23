# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSymmetric(self, root: TreeNode) -> bool:
        def isMirror(root: TreeNode, toor: TreeNode) -> bool:
            if not root and not toor:
                return True
            if not root or not toor or root.val != toor.val:
                return False
            return isMirror(root.left, toor.right) and isMirror(root.right, toor.left)

        return isMirror(root, root)
