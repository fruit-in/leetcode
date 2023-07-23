# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        nodes = [root]
        nodes_sum = []
        ret = 0

        while nodes:
            cur = nodes.pop()
            if cur:
                nodes.append(cur.left)
                nodes.append(cur.right)
                nodes_sum.append((cur, sum))

        while nodes_sum:
            cur, sum = nodes_sum.pop()
            if cur.val == sum:
                ret += 1
            if cur.left:
                nodes_sum.append((cur.left, sum - cur.val))
            if cur.right:
                nodes_sum.append((cur.right, sum - cur.val))

        return ret
