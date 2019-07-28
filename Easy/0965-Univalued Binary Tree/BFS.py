# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isUnivalTree(self, root: TreeNode) -> bool:
        nodes = [root]
        i = 0
        while i < len(nodes):
            if nodes[i].val != root.val:
                return False
            if nodes[i].left:
                nodes.append(nodes[i].left)
            if nodes[i].right:
                nodes.append(nodes[i].right)
            i += 1
        return True
