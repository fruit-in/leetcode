# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        stackp, stackq = [p], [q]
        while stackp:
            nodep, nodeq = stackp.pop(), stackq.pop()
            if not nodep and not nodeq:
                continue
            if not nodep or not nodeq or nodep.val != nodeq.val:
                return False
            stackp.append(nodep.left)
            stackq.append(nodeq.left)
            stackp.append(nodep.right)
            stackq.append(nodeq.right)
        return True
