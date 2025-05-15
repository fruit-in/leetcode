# 1219. Path with Maximum Gold
In a gold mine `grid` of size `m x n`, each cell in this mine has an integer representing the amount of gold in that cell, `0` if it is empty.

Return the maximum amount of gold you can collect under the conditions:
* Every time you are located in a cell you will collect all the gold in that cell.
* From your position, you can walk one step to the left, right, up, or down.
* You can't visit the same cell more than once.
* Never visit a cell with `0` gold.
* You can start and stop collecting gold from **any** position in the grid that has some gold.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[0,6,0],[5,8,7],[0,9,0]]
<strong>Output:</strong> 24
<strong>Explanation:</strong>
[[0,6,0],
 [5,8,7],
 [0,9,0]]
Path to get the maximum gold, 9 -> 8 -> 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
<strong>Output:</strong> 28
<strong>Explanation:</strong>
[[1,0,7],
 [2,0,6],
 [3,4,5],
 [0,3,0],
 [9,0,20]]
Path to get the maximum gold, 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 15`
* `0 <= grid[i][j] <= 100`
* There are at most **25** cells containing gold.

## Solutions (Python)

### 1. Solution
```Python
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
```
