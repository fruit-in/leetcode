# 2373. Largest Local Values in a Matrix
You are given an `n x n` integer matrix `grid`.

Generate an integer matrix `maxLocal` of size `(n - 2) x (n - 2)` such that:

* `maxLocal[i][j]` is equal to the **largest** value of the `3 x 3` matrix in `grid` centered around row `i + 1` and column `j + 1`.

In other words, we want to find the largest value in every contiguous `3 x 3` matrix in `grid`.

Return *the generated matrix*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/06/21/ex1.png)
<pre>
<strong>Input:</strong> grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
<strong>Output:</strong> [[9,9],[8,6]]
<strong>Explanation:</strong> The diagram above shows the original matrix and the generated matrix.
Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/07/02/ex2new2.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
<strong>Output:</strong> [[2,2,2],[2,2,2],[2,2,2]]
<strong>Explanation:</strong> Notice that the 2 is contained within every contiguous 3 x 3 matrix in grid.
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `3 <= n <= 100`
* `1 <= grid[i][j] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];

        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for a in 0..3 {
                    for b in 0..3 {
                        max_local[i][j] = max_local[i][j].max(grid[i + a][j + b]);
                    }
                }
            }
        }

        max_local
    }
}
```
