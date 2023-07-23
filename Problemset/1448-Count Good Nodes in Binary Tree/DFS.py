# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        def foo(root: TreeNode, max_val: int) -> int:
            max_val = max(max_val, root.val)

            ret = 1 if root.val >= max_val else 0
            ret += foo(root.left, max_val) if root.left else 0
            ret += foo(root.right, max_val) if root.right else 0

            return ret

        return foo(root, -10000)
