# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        ret = float('+inf')
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            if root.val < curr.val < ret:
                ret = curr.val
            elif curr.val == root.val and curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        return ret if ret < float('+inf') else -1
