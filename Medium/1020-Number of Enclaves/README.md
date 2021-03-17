# 1020. Number of Enclaves
You are given an `m x n` binary matrix `grid`, where `0` represents a sea cell and `1` represents a land cell.

A **move** consists of walking from one land cell to another adjacent (**4-directionally**) land cell or walking off the boundary of the `grid`.

Return *the number of land cells in* `grid` *for which we cannot walk off the boundary of the grid in any number of **moves***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/02/18/enclaves1.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/02/18/enclaves2.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All 1s are either on the boundary or can reach the boundary.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 500`
* `grid[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. DFS
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        let m = a.len() as i32;
        let n = a[0].len() as i32;
        let mut seen = HashSet::new();
        let mut cells = Vec::new();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if a[i as usize][j as usize] == 1 {
                    if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                        seen.insert((i, j));
                        cells.push((i, j));
                    } else {
                        ret += 1;
                    }
                }
            }
        }

        while let Some((i, j)) = cells.pop() {
            for (x, y) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (i, j) = (i + x, j + y);
                if i > 0
                    && i < m - 1
                    && j > 0
                    && j < n - 1
                    && a[i as usize][j as usize] == 1
                    && !seen.contains(&(i, j))
                {
                    seen.insert((i, j));
                    cells.push((i, j));
                    ret -= 1;
                }
            }
        }

        ret
    }
}
```
