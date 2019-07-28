# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def leafSimilar(self, root1: TreeNode, root2: TreeNode) -> bool:
        def getLeaves(root: TreeNode) -> List[int]:
            if not root.left and not root.right:
                return [root.val]
            leaves = []
            if root.left:
                leaves += getLeaves(root.left)
            if root.right:
                leaves += getLeaves(root.right)
            return leaves

        return getLeaves(root1) == getLeaves(root2)
