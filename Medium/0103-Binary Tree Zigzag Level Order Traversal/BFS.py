# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def zigzagLevelOrder(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []

        curr_level = [root]
        direction = 1
        ret = []

        while curr_level:
            curr_vals = []
            next_level = []

            for node in curr_level:
                curr_vals.append(node.val)
                if node.left:
                    next_level.append(node.left)
                if node.right:
                    next_level.append(node.right)

            ret.append(curr_vals[::direction])
            direction = -direction
            curr_level = next_level

        return ret
