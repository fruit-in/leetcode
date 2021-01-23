# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def bstFromPreorder(self, preorder: List[int]) -> TreeNode:
        if not preorder:
            return None

        return TreeNode(
            preorder[0],
            self.bstFromPreorder(
                [val for val in preorder if val < preorder[0]]),
            self.bstFromPreorder(
                [val for val in preorder if val > preorder[0]])
        )
