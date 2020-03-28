# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def subtreeWithAllDeepest(self, root: TreeNode) -> TreeNode:
        def helper(root: TreeNode) -> (TreeNode, int):
            if not root:
                return None, -1

            l_node, l_h = helper(root.left)
            r_node, r_h = helper(root.right)

            if l_h > r_h:
                return l_node, l_h + 1
            elif l_h < r_h:
                return r_node, r_h + 1
            else:
                return root, l_h + 1

        return helper(root)[0]
