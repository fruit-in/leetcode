# 1254. 统计封闭岛屿的数目
二维矩阵 `grid` 由 `0` （土地）和 `1` （水）组成。岛是由最大的4个方向连通的 `0` 组成的群，封闭岛是一个 `完全` 由1包围（左、上、右、下）的岛。

请返回 *封闭岛屿* 的数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/10/31/sample_3_1610.png)
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
<strong>输出:</strong> 2
<strong>解释:</strong>
灰色区域的岛屿是封闭岛屿，因为这座岛屿完全被水域包围（即被 1 区域包围）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/10/31/sample_4_1610.png)
<pre>
<strong>输入:</strong> grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1,1,1],
             [1,0,0,0,0,0,1],
             [1,0,1,1,1,0,1],
             [1,0,1,0,1,0,1],
             [1,0,1,1,1,0,1],
             [1,0,0,0,0,0,1],
             [1,1,1,1,1,1,1]]
<strong>输出:</strong> 2
</pre>

#### 提示:
* `1 <= grid.length, grid[0].length <= 100`
* `0 <= grid[i][j] <=1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![];
        let mut ret = 0;

        for r in 0..m {
            if grid[r][0] == 0 {
                grid[r][0] = 1;
                stack.push((r, 0));
            }
            if grid[r][n - 1] == 0 {
                grid[r][n - 1] = 1;
                stack.push((r, n - 1));
            }
        }
        for c in 1..n - 1 {
            if grid[0][c] == 0 {
                grid[0][c] = 1;
                stack.push((0, c));
            }
            if grid[m - 1][c] == 0 {
                grid[m - 1][c] = 1;
                stack.push((m - 1, c));
            }
        }

        while let Some((r, c)) = stack.pop() {
            if r > 0 && grid[r - 1][c] == 0 {
                grid[r - 1][c] = 1;
                stack.push((r - 1, c));
            }
            if r < m - 1 && grid[r + 1][c] == 0 {
                grid[r + 1][c] = 1;
                stack.push((r + 1, c));
            }
            if c > 0 && grid[r][c - 1] == 0 {
                grid[r][c - 1] = 1;
                stack.push((r, c - 1));
            }
            if c < n - 1 && grid[r][c + 1] == 0 {
                grid[r][c + 1] = 1;
                stack.push((r, c + 1));
            }
        }

        for r in 1..m - 1 {
            for c in 1..n - 1 {
                if grid[r][c] == 1 {
                    continue;
                }

                grid[r][c] = 1;
                stack.push((r, c));
                while let Some((r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == 0 {
                        grid[r - 1][c] = 1;
                        stack.push((r - 1, c));
                    }
                    if r < m - 1 && grid[r + 1][c] == 0 {
                        grid[r + 1][c] = 1;
                        stack.push((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == 0 {
                        grid[r][c - 1] = 1;
                        stack.push((r, c - 1));
                    }
                    if c < n - 1 && grid[r][c + 1] == 0 {
                        grid[r][c + 1] = 1;
                        stack.push((r, c + 1));
                    }
                }

                ret += 1;
            }
        }

        ret
    }
}
```
