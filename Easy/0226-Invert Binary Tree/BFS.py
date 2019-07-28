# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def invertTree(self, root: TreeNode) -> TreeNode:
        nodes = [root]
        i = 0
        while i < len(nodes):
            if nodes[i]:
                nodes.append(nodes[i].left)
                nodes.append(nodes[i].right)
                nodes[i].left = nodes[-1]
                nodes[i].right = nodes[-2]
            i += 1
        return root
