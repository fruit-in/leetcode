# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rob(self, root: TreeNode) -> int:
        def foo(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            rob_left = foo(root.left)
            rob_right = foo(root.right)

            return (root.val + rob_left[1] + rob_right[1],
                    max(rob_left) + max(rob_right))

        return max(foo(root))
