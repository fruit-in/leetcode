# 1568. Minimum Number of Days to Disconnect Island
You are given an `m x n` binary grid `grid` where `1` represents land and `0` represents water. An **island** is a maximal **4-directionally** (horizontal or vertical) connected group of `1`'s.

The grid is said to be **connected** if we have **exactly one island**, otherwise is said **disconnected**.

In one day, we are allowed to change **any** single land cell `(1)` into a water cell `(0)`.

Return *the minimum number of days to disconnect the grid*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/24/land1.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We need at least 2 days to get a disconnected grid.
Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/24/land2.jpg)
<pre>
<strong>Input:</strong> grid = [[1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Grid of full water is also disconnected ([[1,1]] -> [[0,0]]), 0 islands.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 30`
* `grid[i][j]` is either `0` or `1`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minDays(self, grid: List[List[int]]) -> int:
        def isDisconnected(grid: List[List[int]]) -> bool:
            visited = set()

            for i in range(len(grid)):
                for j in range(len(grid[0])):
                    if grid[i][j] == 0 or (i, j) in visited:
                        continue

                    if visited:
                        return True

                    stack = [(i, j)]
                    visited.add((i, j))

                    while stack:
                        r, c = stack.pop()
                        if r > 0 and grid[r - 1][c] == 1 and (r - 1, c) not in visited:
                            stack.append((r - 1, c))
                            visited.add((r - 1, c))
                        if r < len(grid) - 1 and grid[r + 1][c] == 1 and (r + 1, c) not in visited:
                            stack.append((r + 1, c))
                            visited.add((r + 1, c))
                        if c > 0 and grid[r][c - 1] == 1 and (r, c - 1) not in visited:
                            stack.append((r, c - 1))
                            visited.add((r, c - 1))
                        if c < len(grid[0]) - 1 and grid[r][c + 1] == 1 and (r, c + 1) not in visited:
                            stack.append((r, c + 1))
                            visited.add((r, c + 1))

            return not visited

        if isDisconnected(grid):
            return 0

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == 0:
                    continue

                grid[i][j] = 0
                if isDisconnected(grid):
                    return 1
                grid[i][j] = 1

        return 2
```
