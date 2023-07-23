# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        root_parent = TreeNode(0)
        prev = root_parent
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            prev.right = curr
            curr.left = None
            prev = curr

            curr = curr.right

        return root_parent.right
