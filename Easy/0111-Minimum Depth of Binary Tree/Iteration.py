# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def minDepth(self, root: TreeNode) -> int:
        if not root:
            return 0
        depth = 1
        nodes = [root]
        while True:
            for node in nodes:
                if not node.left and not node.right:
                    return depth
            depth += 1
            nodes = [node.left for node in nodes if node.left] \
                + [node.right for node in nodes if node.right]
