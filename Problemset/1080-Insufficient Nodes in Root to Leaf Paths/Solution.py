# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sufficientSubset(self, root: Optional[TreeNode], limit: int) -> Optional[TreeNode]:
        def dfs(root: Optional[TreeNode]) -> None:
            if root is not None:
                if root.left is not None:
                    root.left.rootsum = root.rootsum + root.left.val
                if root.right is not None:
                    root.right.rootsum = root.rootsum + root.right.val

                dfs(root.left)
                dfs(root.right)

                if root.left is None and root.right is None:
                    root.leafsum = 0
                elif root.left is None:
                    root.leafsum = root.right.leafsum + root.right.val
                elif root.right is None:
                    root.leafsum = root.left.leafsum + root.left.val
                else:
                    root.leafsum = max(
                        root.left.leafsum + root.left.val, root.right.leafsum + root.right.val)

        root.rootsum = root.val
        dfs(root)

        if root.rootsum + root.leafsum < limit:
            return None

        nodes = [root]

        while nodes != []:
            node = nodes.pop()
            if node.left is not None:
                if node.left.rootsum + node.left.leafsum < limit:
                    node.left = None
                else:
                    nodes.append(node.left)
            if node.right is not None:
                if node.right.rootsum + node.right.leafsum < limit:
                    node.right = None
                else:
                    nodes.append(node.right)

        return root
