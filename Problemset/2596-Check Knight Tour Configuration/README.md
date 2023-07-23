# 2596. Check Knight Tour Configuration
There is a knight on an `n x n` chessboard. In a valid configuration, the knight starts **at the top-left cell** of the board and visits every cell on the board **exactly once**.

You are given an `n x n` integer matrix `grid` consisting of distinct integers from the range `[0, n * n - 1]` where `grid[row][col]` indicates that the cell `(row, col)` is the <code>grid[row][col]<sup>th</sup></code> cell that the knight visited. The moves are **0-indexed**.

Return `true` *if* `grid` *represents a valid configuration of the knight's movements or* `false` *otherwise*.

**Note** that a valid knight move consists of moving two squares vertically and one square horizontally, or two squares horizontally and one square vertically. The figure below illustrates all the possible eight moves of a knight from some cell.

![](https://assets.leetcode.com/uploads/2018/10/12/knight.png)

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/28/yetgriddrawio-5.png)
<pre>
<strong>Input:</strong> grid = [[0,11,16,5,20],[17,4,19,10,15],[12,1,8,21,6],[3,18,23,14,9],[24,13,2,7,22]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram represents the grid. It can be shown that it is a valid configuration.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/12/28/yetgriddrawio-6.png)
<pre>
<strong>Input:</strong> grid = [[0,3,6],[5,8,1],[2,7,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> The above diagram represents the grid. The 8th move of the knight is not valid considering its position after the 7th move.
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `3 <= n <= 7`
* `0 <= grid[row][col] < n * n`
* All integers in `grid` are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut movements = vec![(0, 0); grid.len().pow(2)];

        for row in 0..grid.len() {
            for col in 0..grid.len() {
                movements[grid[row][col] as usize] = (row as i32, col as i32);
            }
        }

        if movements[0] != (0, 0) {
            return false;
        }

        for i in 1..movements.len() {
            let x = (movements[i].0 - movements[i - 1].0).abs();
            let y = (movements[i].1 - movements[i - 1].1).abs();

            if x * y != 2 {
                return false;
            }
        }

        true
    }
}
```
