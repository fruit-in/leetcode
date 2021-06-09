# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def delNodes(self, root: TreeNode, to_delete: List[int]) -> List[TreeNode]:
        if root is None:
            return []

        ret = [root]
        left_ret = self.delNodes(root.left, to_delete)
        right_ret = self.delNodes(root.right, to_delete)

        if root.val in to_delete:
            return left_ret + right_ret

        if root.left is not None:
            if root.left.val in to_delete:
                root.left = None
                ret += left_ret
            else:
                ret += left_ret[1:]
        if root.right is not None:
            if root.right.val in to_delete:
                root.right = None
                ret += right_ret
            else:
                ret += right_ret[1:]

        return ret
