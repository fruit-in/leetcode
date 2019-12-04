# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        nodes = []
        curr = root
        vals = []

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()
            vals.append(curr.val)

            curr = curr.right

        l, r = 0, len(vals) - 1

        while l < r:
            if vals[l] + vals[r] < k:
                l += 1
            elif vals[l] + vals[r] > k:
                r -= 1
            else:
                return True

        return False
