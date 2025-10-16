# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfsLevelSums(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            while depth >= len(levelsums):
                levelsums.append(0)
            levelsums[depth] += root.val

            dfsLevelSums(root.left, depth + 1)
            dfsLevelSums(root.right, depth + 1)

        def dfsCousins(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            leftval = root.left.val if root.left else 0
            rightval = root.right.val if root.right else 0

            if root.left:
                root.left.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.left, depth + 1)
            if root.right:
                root.right.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.right, depth + 1)

        levelsums = []
        dfsLevelSums(root, 0)
        dfsCousins(root, 0)
        root.val = 0

        return root
