# 1970. 你能穿过矩阵的最后一天
给你一个下标从 **1** 开始的二进制矩阵，其中 `0` 表示陆地，`1` 表示水域。同时给你 `row` 和 `col` 分别表示矩阵中行和列的数目。

一开始在第 `0` 天，**整个** 矩阵都是 **陆地** 。但每一天都会有一块新陆地被 **水** 淹没变成水域。给你一个下标从 **1** 开始的二维数组 `cells` ，其中 <code>cells[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> 表示在第 `i` 天，第 <code>r<sub>i</sub></code> 行 <code>c<sub>i</sub></code> 列（下标都是从 **1** 开始）的陆地会变成 **水域** （也就是 `0` 变成 `1` ）。

你想知道从矩阵最 **上面** 一行走到最 **下面** 一行，且只经过陆地格子的 **最后一天** 是哪一天。你可以从最上面一行的 **任意** 格子出发，到达最下面一行的 **任意** 格子。你只能沿着 **四个** 基本方向移动（也就是上下左右）。

请返回只经过陆地格子能从最 **上面** 一行走到最 **下面** 一行的 **最后一天** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/27/1.png)
<pre>
<strong>输入:</strong> row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
<strong>输出:</strong> 2
<strong>解释:</strong> 上图描述了矩阵从第 0 天开始是如何变化的。
可以从最上面一行到最下面一行的最后一天是第 2 天。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/27/2.png)
<pre>
<strong>输入:</strong> row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
<strong>输出:</strong> 1
<strong>解释:</strong> 上图描述了矩阵从第 0 天开始是如何变化的。
可以从最上面一行到最下面一行的最后一天是第 1 天。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/07/27/3.png)
<pre>
<strong>输入:</strong> row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 上图描述了矩阵从第 0 天开始是如何变化的。
可以从最上面一行到最下面一行的最后一天是第 3 天。
</pre>

#### 提示:
* <code>2 <= row, col <= 2 * 10<sup>4</sup></code>
* <code>4 <= row * col <= 2 * 10<sup>4</sup></code>
* `cells.length == row * col`
* <code>1 <= r<sub>i</sub> <= row</code>
* <code>1 <= c<sub>i</sub> <= col</code>
* `cells` 中的所有格子坐标都是 **唯一** 的。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def latestDayToCross(self, row: int, col: int, cells: List[List[int]]) -> int:
        def cannotCross(day: int) -> bool:
            grid = [[0] * col for _ in range(row)]
            stack = []
            visited = set()

            for r, c in cells[:day]:
                grid[r - 1][c - 1] = 1

            stack.extend((0, c) for c in range(col) if grid[0][c] == 0)
            visited.update((0, c) for c in range(col) if grid[0][c] == 0)

            while stack != []:
                r, c = stack.pop()

                if r == row - 1:
                    return False

                if r - 1 >= 0 and grid[r - 1][c] == 0 and (r - 1, c) not in visited:
                    stack.append((r - 1, c))
                    visited.add((r - 1, c))
                if r + 1 < row and grid[r + 1][c] == 0 and (r + 1, c) not in visited:
                    stack.append((r + 1, c))
                    visited.add((r + 1, c))
                if c - 1 >= 0 and grid[r][c - 1] == 0 and (r, c - 1) not in visited:
                    stack.append((r, c - 1))
                    visited.add((r, c - 1))
                if c + 1 < col and grid[r][c + 1] == 0 and (r, c + 1) not in visited:
                    stack.append((r, c + 1))
                    visited.add((r, c + 1))

            return True

        return bisect.bisect_left(list(range(len(cells))), True, key=cannotCross) - 1
```
