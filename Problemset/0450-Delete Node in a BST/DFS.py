# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def deleteNode(self, root: TreeNode, key: int) -> TreeNode:
        if root is None:
            return None

        if root.val == key:
            root = self.merge(root.left, root.right)
        elif root.val > key:
            root.left = self.deleteNode(root.left, key)
        else:
            root.right = self.deleteNode(root.right, key)

        return root

    def merge(self, left: TreeNode, right: TreeNode) -> TreeNode:
        if left is None:
            return right

        curr = left
        while curr.right is not None:
            curr = curr.right
        curr.right = right

        return left
