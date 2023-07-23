# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lcaDeepestLeaves(self, root: TreeNode) -> TreeNode:
        def helper(root: TreeNode, depth: int) -> (TreeNode, int):
            if not root:
                return (None, depth - 1)

            l_node, l_depth = helper(root.left, depth + 1)
            r_node, r_depth = helper(root.right, depth + 1)

            if l_depth > r_depth:
                return (l_node, l_depth)
            elif l_depth < r_depth:
                return (r_node, r_depth)
            else:
                return (root, l_depth)

        return helper(root, 0)[0]
