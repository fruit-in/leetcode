# 1895. Largest Magic Square
A `k x k` **magic square** is a `k x k` grid filled with integers such that every row sum, every column sum, and both diagonal sums are **all equal**. The integers in the magic square **do not have to be distinct**. Every `1 x 1` grid is trivially a **magic square**.

Given an `m x n` integer `grid`, return *the **size** (i.e., the side length* `k`*) of the **largest magic square** that can be found within this grid*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/29/magicsquare-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The largest magic square has a size of 3.
Every row sum, column sum, and diagonal sum of this magic square is equal to 12.
- Row sums: 5+1+6 = 5+4+3 = 2+7+3 = 12
- Column sums: 5+5+2 = 1+4+7 = 6+3+3 = 12
- Diagonal sums: 5+4+3 = 6+4+2 = 12
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/29/magicsquare2-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* <code>1 <= grid[i][j] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut prefix_sum_row = vec![vec![0; n + 1]; m];
        let mut prefix_sum_col = vec![vec![0; m + 1]; n];
        let mut ret = 1;

        for i in 0..m {
            for j in 0..n {
                prefix_sum_row[i][j + 1] = prefix_sum_row[i][j] + grid[i][j];
                prefix_sum_col[j][i + 1] = prefix_sum_col[j][i] + grid[i][j];

                for k in (2..=i.min(j) + 1).rev() {
                    if k <= ret {
                        break;
                    }

                    let sum = (0..k).map(|l| grid[i - l][j - l]).sum::<i32>();
                    if (0..k).map(|l| grid[i - l][j + 1 - k + l]).sum::<i32>() != sum {
                        continue;
                    }
                    if (i + 1 - k..=i)
                        .any(|r| prefix_sum_row[r][j + 1] - prefix_sum_row[r][j + 1 - k] != sum)
                    {
                        continue;
                    }
                    if (j + 1 - k..=j)
                        .any(|c| prefix_sum_col[c][i + 1] - prefix_sum_col[c][i + 1 - k] != sum)
                    {
                        continue;
                    }

                    ret = k;
                }
            }
        }

        ret as i32
    }
}
```
