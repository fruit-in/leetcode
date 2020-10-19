# 200. 岛屿数量
给你一个由 `'1'`（陆地）和 `'0'`（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。

#### 示例 1:
<pre>
<b>输入:</b> grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
<b>输出:</b> 1
</pre>

#### 示例 2:
<pre>
<b>输入:</b> grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
<b>输出:</b> 3
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 300`
* `grid[i][j]` 的值为 `'0'` 或 `'1'`.

## 题解 (Python)

### 1. 深度优先搜索
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
