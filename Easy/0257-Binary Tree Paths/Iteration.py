# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def binaryTreePaths(self, root: TreeNode) -> List[str]:
        if not root:
            return []
        paths = []
        nodes = [(root, str(root.val))]
        while nodes:
            curr, path = nodes.pop()
            if not curr.left and not curr.right:
                paths.append(path)
            if curr.left:
                nodes.append((curr.left, path + "->" + str(curr.left.val)))
            if curr.right:
                nodes.append((curr.right, path + "->" + str(curr.right.val)))
        return paths
