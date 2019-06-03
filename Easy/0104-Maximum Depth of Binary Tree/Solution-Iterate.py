# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None


class Solution:
    def maxDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        depth = 0
        nodes = [root]
        while nodes:
            depth += 1
            nodes = [node.left for node in nodes if node.left] \
                + [node.right for node in nodes if node.right]
        return depth
