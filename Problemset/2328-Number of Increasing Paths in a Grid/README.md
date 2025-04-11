# 2328. Number of Increasing Paths in a Grid
You are given an `m x n` integer matrix `grid`, where you can move from a cell to any adjacent cell in all `4` directions.

Return *the number of **strictly increasing** paths in the grid such that you can start from **any** cell and end at **any** cell*. Since the answer may be very large, return it modulo <code>10<sup>9</sup> + 7</code>.

Two paths are considered different if they do not have exactly the same sequence of visited cells.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/05/10/griddrawio-4.png)
<pre>
<strong>Input:</strong> grid = [[1,1],[3,4]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The strictly increasing paths are:
- Paths with length 1: [1], [1], [3], [4].
- Paths with length 2: [1 -> 3], [1 -> 4], [3 -> 4].
- Paths with length 3: [1 -> 3 -> 4].
The total number of paths is 4 + 3 + 1 = 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[1],[2]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The strictly increasing paths are:
- Paths with length 1: [1], [2].
- Paths with length 2: [1 -> 2].
The total number of paths is 2 + 1 = 3.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 1000`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= grid[i][j] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut indices = vec![];
        let mut paths_count = vec![vec![1; n]; m];
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                indices.push((i, j));
            }
        }

        indices.sort_unstable_by_key(|&(i, j)| grid[i][j]);

        for (i, j) in indices {
            if i > 0 && grid[i - 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i - 1][j]) % 1_000_000_007;
            }
            if i < m - 1 && grid[i + 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i + 1][j]) % 1_000_000_007;
            }
            if j > 0 && grid[i][j - 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j - 1]) % 1_000_000_007;
            }
            if j < n - 1 && grid[i][j + 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j + 1]) % 1_000_000_007;
            }

            ret = (ret + paths_count[i][j]) % 1_000_000_007;
        }

        ret
    }
}
```
