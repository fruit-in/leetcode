"""
# Definition for a QuadTree node.
class Node:
    def __init__(self, val, isLeaf, topLeft, topRight, bottomLeft, bottomRight):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight
"""


class Solution:
    def construct(self, grid: List[List[int]]) -> 'Node':
        n = len(grid)
        m = n // 2
        if n == 1:
            return Node(grid[0][0], True, None, None, None, None)

        topLeft = self.construct([grid[i][:m] for i in range(m)])
        topRight = self.construct([grid[i][m:] for i in range(m)])
        bottomLeft = self.construct([grid[i][:m] for i in range(m, n)])
        bottomRight = self.construct([grid[i][m:] for i in range(m, n)])

        if topLeft.val == topRight.val == bottomLeft.val == bottomRight.val \
                and topLeft.isLeaf and topRight.isLeaf and bottomLeft.isLeaf and bottomRight.isLeaf:
            return Node(grid[0][0], True, None, None, None, None)

        return Node(grid[0][0], False, topLeft, topRight, bottomLeft, bottomRight)
