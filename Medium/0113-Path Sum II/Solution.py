# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> List[List[int]]:
        if not root:
            return []

        if not (root.left or root.right) and root.val == sum:
            return [[root.val]]

        paths = self.pathSum(root.left, sum - root.val)
        paths.extend(self.pathSum(root.right, sum - root.val))

        for path in paths:
            path.insert(0, root.val)

        return paths
