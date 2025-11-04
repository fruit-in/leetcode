# 1568. 使陆地分离的最少天数
给你一个大小为 `m x n` ，由若干 `0` 和 `1` 组成的二维网格 `grid` ，其中 `1` 表示陆地， `0` 表示水。**岛屿** 由水平方向或竖直方向上相邻的 1 （陆地）连接形成。

如果 **恰好只有一座岛屿** ，则认为陆地是 **连通的** ；否则，陆地就是 **分离的** 。

一天内，可以将 **任何单个** 陆地单元（`1`）更改为水单元（`0`）。

返回使陆地分离的最少天数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/24/land1.jpg)
<pre>
<strong>输入:</strong> grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]
<strong>输出:</strong> 2
<strong>解释:</strong> 至少需要 2 天才能得到分离的陆地。
将陆地 grid[1][1] 和 grid[0][2] 更改为水，得到两个分离的岛屿。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/24/land2.jpg)
<pre>
<strong>输入:</strong> grid = [[1,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 如果网格中都是水，也认为是分离的 ([[1,1]] -> [[0,0]])，0 岛屿。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 30`
* `grid[i][j]` 为 `0` 或 `1`

## 题解 (Python)

### 1. 题解
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
