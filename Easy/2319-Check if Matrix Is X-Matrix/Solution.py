class Solution:
    def checkXMatrix(self, grid: List[List[int]]) -> bool:
        for x in range(len(grid)):
            for y in range(len(grid)):
                if (x == y or x == len(grid) - 1 - y) ^ (grid[x][y] != 0):
                    return False

        return True
