# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def inorderTraversal(self, root: TreeNode) -> List[int]:
        vals = []
        nodes = []
        node = root
        while node or nodes:
            while node:
                nodes.append(node)
                node = node.left
            node = nodes.pop()
            vals.append(node.val)
            node = node.right
        return vals
