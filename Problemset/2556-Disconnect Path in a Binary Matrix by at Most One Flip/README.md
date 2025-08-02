# 2556. Disconnect Path in a Binary Matrix by at Most One Flip
You are given a **0-indexed** `m x n` **binary** matrix `grid`. You can move from a cell `(row, col)` to any of the cells `(row + 1, col)` or `(row, col + 1)` that has the value `1`. The matrix is **disconnected** if there is no path from `(0, 0)` to `(m - 1, n - 1)`.

You can flip the value of **at most one** (possibly none) cell. You **cannot flip** the cells `(0, 0)` and `(m - 1, n - 1)`.

Return `true` *if it is possible to make the matrix disconnect or* `false` *otherwise*.

**Note** that flipping a cell changes its value from `0` to `1` or from `1` to `0`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/07/yetgrid2drawio.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,0,0],[1,1,1]]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can change the cell shown in the diagram above. There is no path from (0, 0) to (2, 2) in the resulting grid.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/12/07/yetgrid3drawio.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is not possible to change at most one cell such that there is not path from (0, 0) to (2, 2).
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 1000`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `grid[i][j]` is either `0` or `1`.
* `grid[0][0] == grid[m - 1][n - 1] == 1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        if !connect[m - 1][n - 1] {
            return true;
        }

        let mut r = m - 1;
        let mut c = n - 1;

        while r > 0 || c > 0 {
            if r > 0 && connect[r - 1][c] {
                r -= 1;
            } else {
                c -= 1;
            }
            grid[r][c] = 0;
        }

        connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        !connect[m - 1][n - 1]
    }
}
```
