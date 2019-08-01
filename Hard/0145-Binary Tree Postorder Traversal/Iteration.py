# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def postorderTraversal(self, root: TreeNode) -> List[int]:
        nset = set()
        vals = []
        nodes = [None]
        node = root
        
        while nodes and node:
            while node.left and node.left not in nset:
                nodes.append(node)
                node = node.left
            if node.right and node.right not in nset:
                nodes.append(node)
                node = node.right
            else:
                vals.append(node.val)
                nset.add(node)
                node = nodes.pop()
        return vals
