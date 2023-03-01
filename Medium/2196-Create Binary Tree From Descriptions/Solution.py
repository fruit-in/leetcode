# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def createBinaryTree(self, descriptions: List[List[int]]) -> Optional[TreeNode]:
        parents = set()
        children = set()
        nodes = {}

        for parent, child, isleft in descriptions:
            parents.add(parent)
            children.add(child)

            if parent not in nodes:
                nodes[parent] = TreeNode(parent)
            if child not in nodes:
                nodes[child] = TreeNode(child)

            if isleft:
                nodes[parent].left = nodes[child]
            else:
                nodes[parent].right = nodes[child]

        return nodes[(parents - children).pop()]
