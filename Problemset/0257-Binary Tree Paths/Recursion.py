# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []
        if not root.left and not root.right:
            return [str(root.val)]

        paths = self.binaryTreePaths(root.left)
        paths.extend(self.binaryTreePaths(root.right))
        return ["%d->" % root.val + path for path in paths]
