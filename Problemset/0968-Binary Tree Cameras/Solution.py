# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

from functools import cache


class Solution:
    @cache
    def minCameraCover(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        ret = self.coverBySelf(root)

        if root.left is not None:
            ret = min(ret, self.coverBySelf(root.left) +
                      self.minCameraCover(root.right))
        if root.right is not None:
            ret = min(ret, self.coverBySelf(root.right) +
                      self.minCameraCover(root.left))

        return ret

    @cache
    def coverBySelf(self, root: TreeNode) -> int:
        ret = 1
        if root.left is not None:
            ret += self.coverByParent(root.left)
        if root.right is not None:
            ret += self.coverByParent(root.right)

        return ret

    @cache
    def coverByParent(self, root: TreeNode) -> int:
        return min(self.coverBySelf(root), self.minCameraCover(root.left) + self.minCameraCover(root.right))
