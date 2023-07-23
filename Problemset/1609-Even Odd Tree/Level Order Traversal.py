# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isEvenOddTree(self, root: TreeNode) -> bool:
        even_index = True
        curr_level = [root]

        while curr_level != []:
            prev_val = 0 if even_index else 1_000_001
            next_level = []

            for node in curr_level:
                if (even_index and
                    (node.val % 2 == 0 or prev_val >= node.val)) or \
                        (not even_index and
                         (node.val % 2 == 1 or prev_val <= node.val)):
                    return False

                prev_val = node.val
                if node.left is not None:
                    next_level.append(node.left)
                if node.right is not None:
                    next_level.append(node.right)

            even_index = not even_index
            curr_level = next_level

        return True
