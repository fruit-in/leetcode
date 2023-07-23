# 1139. 最大的以 1 为边界的正方形
给你一个由若干 `0` 和 `1` 组成的二维网格 `grid`，请你找出边界全部由 `1` 组成的最大 **正方形** 子网格，并返回该子网格中的元素数量。如果不存在，则返回 `0`。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[1,1,1],[1,0,1],[1,1,1]]
<strong>输出:</strong> 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,1,0,0]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= grid.length <= 100`
* `1 <= grid[0].length <= 100`
* `grid[i][j]` 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut prefix_sum_row = vec![vec![0; col_len]; row_len];
        let mut prefix_sum_col = vec![vec![0; col_len]; row_len];
        let mut max_len = 0;

        for row in 0..row_len {
            for col in 0..col_len {
                if grid[row][col] == 0 {
                    continue;
                }

                if col > 0 {
                    prefix_sum_row[row][col] = prefix_sum_row[row][col - 1];
                }
                if row > 0 {
                    prefix_sum_col[row][col] = prefix_sum_col[row - 1][col];
                }
                prefix_sum_row[row][col] += 1;
                prefix_sum_col[row][col] += 1;

                for len in
                    (max_len + 1..=prefix_sum_row[row][col].min(prefix_sum_col[row][col])).rev()
                {
                    if prefix_sum_row[row + 1 - len][col] >= len
                        && prefix_sum_col[row][col + 1 - len] >= len
                    {
                        max_len = len;
                        break;
                    }
                }
            }
        }

        (max_len * max_len) as i32
    }
}
```
