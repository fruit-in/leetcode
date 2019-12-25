# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        prev = None
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            new_node = TreeNode(curr.val)
            new_node.right = prev
            prev = new_node

            curr = curr.left

        return prev
