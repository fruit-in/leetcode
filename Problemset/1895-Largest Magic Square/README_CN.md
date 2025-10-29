# 1895. 最大的幻方
一个 `k x k` 的 **幻方** 指的是一个 `k x k` 填满整数的方格阵，且每一行、每一列以及两条对角线的和 **全部相等** 。幻方中的整数 **不需要互不相同** 。显然，每个 `1 x 1` 的方格都是一个幻方。

给你一个 `m x n` 的整数矩阵 `grid` ，请你返回矩阵中 **最大幻方** 的 **尺寸** （即边长 `k`）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/29/magicsquare-grid.jpg)
<pre>
<strong>输入:</strong> grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]
<strong>输出:</strong> 3
<strong>解释:</strong> 最大幻方尺寸为 3 。
每一行，每一列以及两条对角线的和都等于 12 。
- 每一行的和：5+1+6 = 5+4+3 = 2+7+3 = 12
- 每一列的和：5+5+2 = 1+4+7 = 6+3+3 = 12
- 对角线的和：5+4+3 = 6+4+2 = 12
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/05/29/magicsquare2-grid.jpg)
<pre>
<strong>输入:</strong> grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]]
<strong>输出:</strong> 2
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* <code>1 <= grid[i][j] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
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
