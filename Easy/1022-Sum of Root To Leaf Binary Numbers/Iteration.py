# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        nodes = [(root, 0)]
        nums = []

        while nodes:
            cur, num = nodes.pop()

            if not cur.left and not cur.right:
                nums.append((num << 1) + cur.val)
            if cur.left:
                nodes.append((cur.left, (num << 1) + cur.val))
            if cur.right:
                nodes.append((cur.right, (num << 1) + cur.val))

        return sum(nums)
