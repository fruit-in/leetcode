# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        def rootPathSum(root: TreeNode, sum: int) -> int:
            if not root:
                return 0

            ret = 1 if root.val == sum else 0

            ret += rootPathSum(root.left, sum - root.val)
            ret += rootPathSum(root.right, sum - root.val)

            return ret


        if not root:
            return 0

        ret = rootPathSum(root, sum)

        ret += self.pathSum(root.left, sum)
        ret += self.pathSum(root.right, sum)

        return ret
