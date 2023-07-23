# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def deepestLeavesSum(self, root: TreeNode) -> int:
        curr = [root]

        while True:
            next = [n.left for n in curr if n.left]
            next.extend(n.right for n in curr if n.right)

            if not next:
                return sum(n.val for n in curr)

            curr = next
