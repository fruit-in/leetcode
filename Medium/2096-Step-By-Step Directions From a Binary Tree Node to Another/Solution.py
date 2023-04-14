# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def getDirections(self, root: Optional[TreeNode], startValue: int, destValue: int) -> str:
        root2Start = self.root2(root, startValue)
        root2Dest = self.root2(root, destValue)
        minPathLen = min(len(root2Start), len(root2Dest))

        for i in range(minPathLen + 1):
            if i == minPathLen or root2Start[i] != root2Dest[i]:
                return 'U' * (len(root2Start) - i) + root2Dest[i:]

    def root2(self, root: Optional[TreeNode], destValue: int) -> Optional[str]:
        if root is None:
            return None

        if root.val == destValue:
            return ""

        root2Left = self.root2(root.left, destValue)
        if root2Left is not None:
            return 'L' + root2Left

        root2Right = self.root2(root.right, destValue)
        if root2Right is not None:
            return 'R' + root2Right

        return None
