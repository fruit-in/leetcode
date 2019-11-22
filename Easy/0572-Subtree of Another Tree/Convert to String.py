# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        def toString(root: TreeNode) -> str:
            if not root:
                return 'N'
            return "~%d%s%s" % (root.val, toString(root.left), toString(root.right))


        return toString(t) in toString(s)
