class Solution:
    def getMaximumGold(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        gold = 0
        ret = 0

        def dfs(i: int, j: int) -> None:
            nonlocal gold, ret
            ret = max(ret, gold)

            if i > 0 and grid[i - 1][j] > 0:
                gold += grid[i - 1][j]
                grid[i - 1][j] = -grid[i - 1][j]
                dfs(i - 1, j)
                gold += grid[i - 1][j]
                grid[i - 1][j] = -grid[i - 1][j]
            if i + 1 < m and grid[i + 1][j] > 0:
                gold += grid[i + 1][j]
                grid[i + 1][j] = -grid[i + 1][j]
                dfs(i + 1, j)
                gold += grid[i + 1][j]
                grid[i + 1][j] = -grid[i + 1][j]
            if j > 0 and grid[i][j - 1] > 0:
                gold += grid[i][j - 1]
                grid[i][j - 1] = -grid[i][j - 1]
                dfs(i, j - 1)
                gold += grid[i][j - 1]
                grid[i][j - 1] = -grid[i][j - 1]
            if j + 1 < n and grid[i][j + 1] > 0:
                gold += grid[i][j + 1]
                grid[i][j + 1] = -grid[i][j + 1]
                dfs(i, j + 1)
                gold += grid[i][j + 1]
                grid[i][j + 1] = -grid[i][j + 1]

        for i in range(m):
            for j in range(n):
                if grid[i][j] > 0:
                    gold = grid[i][j]
                    grid[i][j] = -grid[i][j]
                    dfs(i, j)
                    grid[i][j] = -grid[i][j]

        return ret
