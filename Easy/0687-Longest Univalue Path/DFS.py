# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def longestUnivaluePath(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int):
            if not root:
                return (0, 0)

            longer, longest = 1, 0
            l = helper(root.left)
            r = helper(root.right)

            if root.left and root.val == root.left.val:
                longer = l[0] + 1
                longest += l[0]
            if root.right and root.val == root.right.val:
                longer = max(longer, r[0] + 1)
                longest += r[0]

            return (longer, max(l[1], r[1], longest))

        return helper(root)[1]
