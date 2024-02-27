# 1034. 边界着色
给你一个大小为 `m x n` 的整数矩阵 `grid` ，表示一个网格。另给你三个整数 `row`、`col` 和 `color` 。网格中的每个值表示该位置处的网格块的颜色。

如果两个方块在任意 4 个方向上相邻，则称它们 **相邻** 。

如果两个方块具有相同的颜色且相邻，它们则属于同一个 **连通分量** 。

**连通分量的边界** 是指连通分量中满足下述条件之一的所有网格块：

* 在上、下、左、右任意一个方向上与不属于同一连通分量的网格块相邻
* 在网格的边界上（第一行/列或最后一行/列）

请你使用指定颜色 `color` 为所有包含网格块 `grid[row][col]` 的 **连通分量的边界** 进行着色。

并返回最终的网格 `grid` 。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
<strong>输出:</strong> [[3,3],[3,2]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
<strong>输出:</strong> [[1,3,3],[2,3,3]]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
<strong>输出:</strong> [[2,2,2],[2,1,2],[2,2,2]]
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* `1 <= grid[i][j], color <= 1000`
* `0 <= row < m`
* `0 <= col < n`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let row = row as usize;
        let col = col as usize;
        let mut stack = vec![(row, col)];
        let mut visited = HashSet::from([(row, col)]);
        let mut borders = vec![];

        while let Some((r, c)) = stack.pop() {
            let mut count = 0;

            if r > 0 && grid[r - 1][c] == grid[row][col] {
                count += 1;
                if visited.insert((r - 1, c)) {
                    stack.push((r - 1, c));
                }
            }
            if r < m - 1 && grid[r + 1][c] == grid[row][col] {
                count += 1;
                if visited.insert((r + 1, c)) {
                    stack.push((r + 1, c));
                }
            }
            if c > 0 && grid[r][c - 1] == grid[row][col] {
                count += 1;
                if visited.insert((r, c - 1)) {
                    stack.push((r, c - 1));
                }
            }
            if c < n - 1 && grid[r][c + 1] == grid[row][col] {
                count += 1;
                if visited.insert((r, c + 1)) {
                    stack.push((r, c + 1));
                }
            }

            if count < 4 {
                borders.push((r, c));
            }
        }

        for (r, c) in borders {
            grid[r][c] = color;
        }

        grid
    }
}
```
