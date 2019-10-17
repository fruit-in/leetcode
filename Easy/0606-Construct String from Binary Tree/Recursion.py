# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = str(t.val)

        if t.left or t.right:
            ret += "(" + self.tree2str(t.left) + ")"
        if t.right:
            ret += "(" + self.tree2str(t.right) + ")"

        return ret
