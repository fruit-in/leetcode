# 2088. Count Fertile Pyramids in a Land
A farmer has a **rectangular grid** of land with `m` rows and `n` columns that can be divided into unit cells. Each cell is either **fertile** (represented by a `1`) or **barren** (represented by a `0`). All cells outside the grid are considered barren.

A **pyramidal plot** of land can be defined as a set of cells with the following criteria:

1. The number of cells in the set has to be **greater than** `1` and all cells must be **fertile**.
2. The **apex** of a pyramid is the **topmost** cell of the pyramid. The **height** of a pyramid is the number of rows it covers. Let `(r, c)` be the apex of the pyramid, and its height be `h`. Then, the plot comprises of cells `(i, j)` where `r <= i <= r + h - 1` **and** `c - (i - r) <= j <= c + (i - r)`.

An **inverse pyramidal plot** of land can be defined as a set of cells with similar criteria:

1. The number of cells in the set has to be **greater than** `1` and all cells must be **fertile**.
2. The **apex** of an inverse pyramid is the **bottommost** cell of the inverse pyramid. The **height** of an inverse pyramid is the number of rows it covers. Let `(r, c)` be the apex of the pyramid, and its height be `h`. Then, the plot comprises of cells `(i, j)` where `r - h + 1 <= i <= r` **and** `c - (r - i) <= j <= c + (r - i)`.

Some examples of valid and invalid pyramidal (and inverse pyramidal) plots are shown below. Black cells indicate fertile cells.

![](https://assets.leetcode.com/uploads/2021/11/08/image.png)

Given a **0-indexed** `m x n` binary matrix `grid` representing the farmland, return *the **total number** of pyramidal and inverse pyramidal plots that can be found in* `grid`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/22/1.JPG)
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[1,1,1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 2 possible pyramidal plots are shown in blue and red respectively.
There are no inverse pyramidal plots in this grid.
Hence total number of pyramidal and inverse pyramidal plots is 2 + 0 = 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/22/2.JPG)
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The pyramidal plot is shown in blue, and the inverse pyramidal plot is shown in red.
Hence the total number of plots is 1 + 1 = 2.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/22/3.JPG)
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,0],[1,1,1,1,1],[1,1,1,1,1],[0,1,0,0,1]]
<strong>Output:</strong> 13
<strong>Explanation:</strong> There are 7 pyramidal plots, 3 of which are shown in the 2nd and 3rd figures.
There are 6 inverse pyramidal plots, 2 of which are shown in the last figure.
The total number of plots is 7 + 6 = 13.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 1000`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `grid[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ret = 0;

        for i in 1..m {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    continue;
                }

                dp[i][j] = grid[i - 1][j - 1] * grid[i - 1][j] * grid[i - 1][j + 1];
                dp[i][j] += dp[i][j] * dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j + 1]);
                ret += dp[i][j];
            }
        }

        dp = vec![vec![0; n]; m];

        for i in (0..m - 1).rev() {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    continue;
                }

                dp[i][j] = grid[i + 1][j - 1] * grid[i + 1][j] * grid[i + 1][j + 1];
                dp[i][j] += dp[i][j] * dp[i + 1][j - 1].min(dp[i + 1][j]).min(dp[i + 1][j + 1]);
                ret += dp[i][j];
            }
        }

        ret
    }
}
```
