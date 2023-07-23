# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths(self, root: TreeNode) -> int:
        def foo(root: TreeNode, x: int) -> int:
            x ^= 1 << root.val

            if not root.left and not root.right:
                return 1 if bin(x).count('1') < 2 else 0
            elif not root.left:
                return foo(root.right, x)
            elif not root.right:
                return foo(root.left, x)
            else:
                return foo(root.left, x) + foo(root.right, x)

        return foo(root, 0)
