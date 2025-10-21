# 2617. 网格图中最少访问的格子数
给你一个下标从 **0** 开始的 `m x n` 整数矩阵 `grid` 。你一开始的位置在 **左上角** 格子 `(0, 0)` 。

当你在格子 `(i, j)` 的时候，你可以移动到以下格子之一：
* 满足 `j < k <= grid[i][j] + j` 的格子 `(i, k)` （向右移动），或者
* 满足 `i < k <= grid[i][j] + i` 的格子 `(k, j)` （向下移动）。

请你返回到达 **右下角** 格子 `(m - 1, n - 1)` 需要经过的最少移动格子数，如果无法到达右下角格子，请你返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/01/25/ex1.png)
<pre>
<strong>输入:</strong> grid = [[3,4,2,1],[4,2,3,1],[2,1,0,0],[2,4,0,0]]
<strong>输出:</strong> 4
<strong>解释:</strong> 上图展示了到达右下角格子经过的 4 个格子。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2023/01/25/ex2.png)
<pre>
<strong>输入:</strong> grid = [[3,4,2,1],[4,2,1,1],[2,1,1,0],[3,4,1,0]]
<strong>输出:</strong> 3
<strong>解释:</strong> 上图展示了到达右下角格子经过的 3 个格子。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2023/01/26/ex3.png)
<pre>
<strong>输入:</strong> grid = [[2,1,0],[1,0,0]]
<strong>输出:</strong> -1
<strong>解释:</strong> 无法到达右下角格子。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `0 <= grid[i][j] < m * n`
* `grid[m - 1][n - 1] == 0`

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut col_heaps = vec![BinaryHeap::new(); n];
        let mut min_visit = vec![vec![-1; n]; m];
        min_visit[0][0] = 1;

        for i in 0..m {
            let mut row_heap = BinaryHeap::new();

            for j in 0..n {
                while let Some(&(Reverse(visit), k)) = row_heap.peek() {
                    if k < j {
                        row_heap.pop();
                    } else {
                        if min_visit[i][j] == -1 || min_visit[i][j] > visit {
                            min_visit[i][j] = visit;
                        }
                        break;
                    }
                }
                while let Some(&(Reverse(visit), k)) = col_heaps[j].peek() {
                    if k < i {
                        col_heaps[j].pop();
                    } else {
                        if min_visit[i][j] == -1 || min_visit[i][j] > visit {
                            min_visit[i][j] = visit;
                        }
                        break;
                    }
                }

                if min_visit[i][j] != -1 {
                    row_heap.push((Reverse(min_visit[i][j] + 1), grid[i][j] as usize + j));
                    col_heaps[j].push((Reverse(min_visit[i][j] + 1), grid[i][j] as usize + i));
                }
            }
        }

        min_visit[m - 1][n - 1]
    }
}
```
