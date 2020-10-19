# 200. Number of Islands
Given an `m x n` 2d `grid` map of `'1'`s (land) and `'0'`s (water), return *the number of islands*.

An **island** is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

#### Example 1:
<pre>
<b>Input:</b> grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
<b>Output:</b> 1
</pre>

#### Example 2:
<pre>
<b>Input:</b> grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
<b>Output:</b> 3
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 300`
* `grid[i][j]` is `'0'` or `'1'`.

## Solutions (Python)

### 1. DFS
```Python
class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        stack = []
        ret = 0

        for r0 in range(m):
            for c0 in range(n):
                if grid[r0][c0] == '1':
                    ret += 1
                    grid[r0][c0] = '0'
                    stack.append((r0, c0))
                    while stack:
                        r1, c1 = stack.pop()
                        for (i, j) in [(0, -1), (0, 1), (1, 0), (-1, 0)]:
                            r2, c2 = r1 + i, c1 + j
                            if 0 <= r2 < m and 0 <= c2 < n and grid[r2][c2] == '1':
                                grid[r2][c2] = '0'
                                stack.append((r2, c2))

        return ret
```
