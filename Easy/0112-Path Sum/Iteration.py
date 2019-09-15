# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def hasPathSum(self, root: TreeNode, sum: int) -> bool:
        if not root:
            return False
        root.val = sum - root.val
        nodes = [root]
        while nodes:
            curr = nodes.pop()
            if curr.val == 0 and not curr.left and not curr.right:
                return True
            if curr.left:
                curr.left.val = curr.val - curr.left.val
                nodes.append(curr.left)
            if curr.right:
                curr.right.val = curr.val - curr.right.val
                nodes.append(curr.right)
        return False
