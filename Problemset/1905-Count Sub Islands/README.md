# 1905. Count Sub Islands
You are given two `m x n` binary matrices `grid1` and `grid2` containing only `0`'s (representing water) and `1`'s (representing land). An **island** is a group of `1`'s connected **4-directionally** (horizontal or vertical). Any cells outside of the grid are considered water cells.

An island in `grid2` is considered a **sub-island** if there is an island in `grid1` that contains **all** the cells that make up **this** island in `grid2`.

Return the ***number** of islands in* `grid2` *that are considered **sub-islands***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/10/test1.png)
<pre>
<strong>Input:</strong> grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/03/testcasex2.png)
<pre>
<strong>Input:</strong> grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.
</pre>

#### Constraints:
* `m == grid1.length == grid2.length`
* `n == grid1[i].length == grid2[i].length`
* `1 <= m, n <= 500`
* `grid1[i][j]` and `grid2[i][j]` are either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid1.len();
        let n = grid1[0].len();
        let mut grid2 = grid2;
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 0 {
                    continue;
                }

                let mut cells = vec![(i, j)];
                let mut is_sub = true;
                grid2[i][j] = 0;

                while let Some((i, j)) = cells.pop() {
                    is_sub &= grid1[i][j] == 1;

                    if i > 0 && grid2[i - 1][j] == 1 {
                        cells.push((i - 1, j));
                        grid2[i - 1][j] = 0;
                    }
                    if i + 1 < m && grid2[i + 1][j] == 1 {
                        cells.push((i + 1, j));
                        grid2[i + 1][j] = 0;
                    }
                    if j > 0 && grid2[i][j - 1] == 1 {
                        cells.push((i, j - 1));
                        grid2[i][j - 1] = 0;
                    }
                    if j + 1 < n && grid2[i][j + 1] == 1 {
                        cells.push((i, j + 1));
                        grid2[i][j + 1] = 0;
                    }
                }

                ret += is_sub as i32;
            }
        }

        ret
    }
}
```
