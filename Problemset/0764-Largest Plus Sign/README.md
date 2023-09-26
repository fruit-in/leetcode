# 764. Largest Plus Sign
You are given an integer `n`. You have an `n x n` binary grid `grid` with all values initially `1`'s except for some indices given in the array `mines`. The <code>i<sup>th</sup></code> element of the array `mines` is defined as <code>mines[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> where <code>grid[x<sub>i</sub>][y<sub>i</sub>] == 0</code>.

Return *the order of the largest **axis-aligned** plus sign of 1's contained in* grid. If there is none, return `0`.

An **axis-aligned plus sign** of `1`'s of order `k` has some center `grid[r][c] == 1` along with four arms of length `k - 1` going up, down, left, and right, and made of `1`'s. Note that there could be `0`'s or `1`'s beyond the arms of the plus sign, only the relevant area of the plus sign is checked for `1`'s.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/13/plus1-grid.jpg)
<pre>
<strong>Input:</strong> n = 5, mines = [[4,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> In the above grid, the largest plus sign can only be of order 2. One of them is shown.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1, mines = [[0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no plus sign, so return 0.
</pre>

#### Constraints:
* `1 <= n <= 500`
* `1 <= mines.length <= 5000`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> < n</code>
* All the pairs <code>(x<sub>i</sub>, y<sub>i</sub>)</code> are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![[1; 5]; n]; n];
        let mut ret = 0;

        for mine in &mines {
            grid[mine[0] as usize][mine[1] as usize] = [0; 5];
        }

        for x in 0..n {
            for y in 1..n {
                if grid[x][y][0] == 1 {
                    grid[x][y][1] += grid[x][y - 1][1];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][2] += grid[y - 1][x][2];
                }
            }
            for y in (0..n - 1).rev() {
                if grid[x][y][0] == 1 {
                    grid[x][y][3] += grid[x][y + 1][3];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][4] += grid[y + 1][x][4];
                }
            }
        }

        for x in 0..n {
            for y in 0..n {
                if grid[x][y][0] == 1 {
                    ret = ret.max(*grid[x][y][1..].iter().min().unwrap());
                }
            }
        }

        ret
    }
}
```
