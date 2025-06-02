# 1536. Minimum Swaps to Arrange a Binary Grid
Given an `n x n` binary `grid`, in one step you can choose two **adjacent rows** of the grid and swap them.

A grid is said to be **valid** if all the cells above the main diagonal are **zeros**.

Return *the minimum number of steps* needed to make the grid valid, or **-1** if the grid cannot be valid.

The main diagonal of a grid is the diagonal that starts at cell `(1, 1)` and ends at cell `(n, n)`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/28/fw.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,1],[1,1,0],[1,0,0]]
<strong>Output:</strong> 3
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/07/16/e2.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[0,1,1,0],[0,1,1,0],[0,1,1,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> All rows are similar, swaps have no effect on the grid.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/07/16/e3.jpg)
<pre>
<strong>Input:</strong> grid = [[1,0,0],[1,1,0],[1,1,1]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `1 <= n <= 200`
* `grid[i][j]` is either `0` or `1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zeros = vec![0; n];
        let mut ret = 0;

        for i in 0..n {
            for j in (0..n).rev() {
                if grid[i][j] == 0 {
                    zeros[i] += 1;
                } else {
                    break;
                }
            }
        }

        for i in 0..n {
            if let Some(j) = zeros.iter().position(|&x| x >= n - 1 - i) {
                zeros.remove(j);
                ret += j;
            } else {
                return -1;
            }
        }

        ret as i32
    }
}
```
