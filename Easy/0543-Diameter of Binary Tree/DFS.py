# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (-1, 0)

            l_longer, l_diameter = helper(root.left)
            r_longer, r_diameter = helper(root.right)

            l_longer += 1
            r_longer += 1

            return (max(l_longer, r_longer), max(l_diameter, r_diameter, l_longer + r_longer))


        return helper(root)[1]
