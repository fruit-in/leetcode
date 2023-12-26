# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isCompleteTree(self, root: Optional[TreeNode]) -> bool:
        nodes = [root]
        depth = 0

        while True:
            children = []

            if nodes[0] == None:
                return True

            if len(nodes) != 2 ** depth:
                return False

            for node in nodes:
                if node is not None:
                    children.append(node.left)
                    children.append(node.right)

            for i in range(1, len(children)):
                if children[i - 1] is None and children[i] is not None:
                    return False

            nodes = children
            depth += 1
