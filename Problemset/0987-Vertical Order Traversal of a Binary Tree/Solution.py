# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> List[List[int]]:
        nodes = [(root, 0, 0)]
        colvals = {}
        ret = []

        while nodes != []:
            curr, row, col = nodes.pop()
            if col not in colvals:
                colvals[col] = []
            colvals[col].append((row, curr.val))

            if curr.left is not None:
                nodes.append((curr.left, row + 1, col - 1))
            if curr.right is not None:
                nodes.append((curr.right, row + 1, col + 1))

        for rowvals in colvals.values():
            rowvals.sort()

        for _, rowvals in sorted(colvals.items()):
            rowvals.sort()
            ret.append([val for _, val in rowvals])

        return ret
