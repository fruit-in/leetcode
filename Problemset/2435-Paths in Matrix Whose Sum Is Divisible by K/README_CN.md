# 2435. 矩阵中和能被 K 整除的路径
给你一个下标从 **0** 开始的 `m x n` 整数矩阵 `grid` 和一个整数 `k` 。你从起点 `(0, 0)` 出发，每一步只能往 **下** 或者往 **右** ，你想要到达终点 `(m - 1, n - 1)` 。

请你返回路径和能被 `k` 整除的路径数目，由于答案可能很大，返回答案对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/08/13/image-20220813183124-1.png)
<pre>
<strong>输入:</strong> grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 有两条路径满足路径上元素的和能被 k 整除。
第一条路径为上图中用红色标注的路径，和为 5 + 2 + 4 + 5 + 2 = 18 ，能被 3 整除。
第二条路径为上图中用蓝色标注的路径，和为 5 + 3 + 0 + 5 + 2 = 15 ，能被 3 整除。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/08/17/image-20220817112930-3.png)
<pre>
<strong>输入:</strong> grid = [[0,0]], k = 5
<strong>输出:</strong> 1
<strong>解释:</strong> 红色标注的路径和为 0 + 0 = 0 ，能被 5 整除。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/08/12/image-20220812224605-3.png)
<pre>
<strong>输入:</strong> grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
<strong>输出:</strong> 10
<strong>解释:</strong> 每个数字都能被 1 整除，所以每一条路径的和都能被 k 整除。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 5 * 10<sup>4</sup></code>
* <code>1 <= m * n <= 5 * 10<sup>4</sup></code>
* `0 <= grid[i][j] <= 100`
* `1 <= k <= 50`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut prev_row_mod = vec![VecDeque::from(vec![0; k]); n];
        prev_row_mod[0][0] = 1;

        for r in 0..m {
            let mut row_mod = prev_row_mod.clone();
            row_mod[0].rotate_right(grid[r][0] as usize % k);

            for c in 1..n {
                for i in 0..k {
                    row_mod[c][i] = (row_mod[c][i] + row_mod[c - 1][i]) % 1_000_000_007;
                }
                row_mod[c].rotate_right(grid[r][c] as usize % k);
            }

            prev_row_mod = row_mod;
        }

        prev_row_mod[n - 1][0]
    }
}
```
