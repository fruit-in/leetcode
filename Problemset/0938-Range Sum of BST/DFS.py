# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def rangeSumBST(self, root: TreeNode, L: int, R: int) -> int:
        rangesum = root.val
        if rangesum < L or rangesum > R:
            rangesum = 0
        if root.left:
            rangesum += self.rangeSumBST(root.left, L, R)
        if root.right:
            rangesum += self.rangeSumBST(root.right, L, R)
        return rangesum
