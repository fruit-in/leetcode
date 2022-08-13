# 1139. Largest 1-Bordered Square
Given a 2D `grid` of `0`s and `1`s, return the number of elements in the largest **square** subgrid that has all `1`s on its **border**, or `0` if such a subgrid doesn't exist in the `grid`.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> 9
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[1,1,0,0]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= grid.length <= 100`
* `1 <= grid[0].length <= 100`
* `grid[i][j]` is `0` or `1`

## Solutions (Rust)

### 1. Solution
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
