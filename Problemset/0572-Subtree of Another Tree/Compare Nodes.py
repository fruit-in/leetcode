# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        def isEqual(s: TreeNode, t: TreeNode) -> bool:
            if s and t and s.val == t.val:
                return isEqual(s.left, t.left) and isEqual(s.right, t.right)
            elif not s and not t:
                return True
            else:
                return False


        return s and (isEqual(s, t) or self.isSubtree(s.left, t) or self.isSubtree(s.right, t))
