# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def recoverTree(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        curr = root
        node1, node2 = None, TreeNode(-inf)
        wrong1, wrong2 = None, None

        while curr:
            if not curr.left:
                node1, node2 = node2, curr
                if not wrong1 and node1.val > node2.val:
                    wrong1 = node1
                if node1.val > node2.val:
                    wrong2 = node2
                curr = curr.right
            else:
                temp = curr.left
                while temp.right and temp.right != curr:
                    temp = temp.right
                if not temp.right:
                    temp.right = curr
                    curr = curr.left
                else:
                    node1, node2 = node2, curr
                    if not wrong1 and node1.val > node2.val:
                        wrong1 = node1
                    if node1.val > node2.val:
                        wrong2 = node2
                    temp.right = None
                    curr = curr.right

        wrong1.val, wrong2.val = wrong2.val, wrong1.val
