# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxProduct(self, root: Optional[TreeNode]) -> int:
        def subtreeSum(root: Optional[TreeNode]) -> int:
            if root is None:
                return 0

            root.val += subtreeSum(root.left)
            root.val += subtreeSum(root.right)

            return root.val

        subtreeSum(root)

        nodes = [root]
        ret = 0

        while nodes != []:
            curr = nodes.pop()
            ret = max(ret, curr.val * (root.val - curr.val))

            if curr.left is not None:
                nodes.append(curr.left)
            if curr.right is not None:
                nodes.append(curr.right)

        return ret % 1000000007
