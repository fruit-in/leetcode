# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        if not root:
            return 0

        nodes_sum = [(root, [sum])]
        ret = 0

        while nodes_sum:
            cur, s = nodes_sum.pop()

            ret += s.count(cur.val)

            s = [x - cur.val for x in s]
            s.append(sum)

            if cur.left:
                nodes_sum.append((cur.left, s))
            if cur.right:
                nodes_sum.append((cur.right, s))

        return ret
