# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def widthOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        currlevel = [(root, 0)]
        ret = 1

        while currlevel != []:
            nextlevel = []
            ret = max(ret, currlevel[-1][1] - currlevel[0][1] + 1)

            for node, x in currlevel:
                if node.left is not None:
                    nextlevel.append((node.left, x << 1))
                if node.right is not None:
                    nextlevel.append((node.right, (x << 1) + 1))

            currlevel = nextlevel

        return ret
