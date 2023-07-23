# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode, target: TreeNode) -> TreeNode:
        nodes = [original, cloned]

        while nodes:
            clo_curr = nodes.pop()
            ori_curr = nodes.pop()

            if ori_curr == target:
                return clo_curr

            if ori_curr.left:
                nodes.append(ori_curr.left)
                nodes.append(clo_curr.left)
            if ori_curr.right:
                nodes.append(ori_curr.right)
                nodes.append(clo_curr.right)
