# 2617. Minimum Number of Visited Cells in a Grid
You are given a **0-indexed** `m x n` integer matrix `grid`. Your initial position is at the **top-left** cell `(0, 0)`.

Starting from the cell `(i, j)`, you can move to one of the following cells:
* Cells `(i, k)` with `j < k <= grid[i][j] + j` (rightward movement), or
* Cells `(k, j)` with `i < k <= grid[i][j] + i` (downward movement).

Return *the minimum number of cells you need to visit to reach the **bottom-right** cell* `(m - 1, n - 1)`. If there is no valid path, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2023/01/25/ex1.png)
<pre>
<strong>Input:</strong> grid = [[3,4,2,1],[4,2,3,1],[2,1,0,0],[2,4,0,0]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The image above shows one of the paths that visits exactly 4 cells.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2023/01/25/ex2.png)
<pre>
<strong>Input:</strong> grid = [[3,4,2,1],[4,2,1,1],[2,1,1,0],[3,4,1,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The image above shows one of the paths that visits exactly 3 cells.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2023/01/26/ex3.png)
<pre>
<strong>Input:</strong> grid = [[2,1,0],[1,0,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be proven that no path exists.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `0 <= grid[i][j] < m * n`
* `grid[m - 1][n - 1] == 0`

## Solutions (Rust)

### 1. Solution
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
