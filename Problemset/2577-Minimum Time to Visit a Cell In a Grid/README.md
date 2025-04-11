# 2577. Minimum Time to Visit a Cell In a Grid
You are given a `m x n` matrix `grid` consisting of **non-negative** integers where `grid[row][col]` represents the **minimum** time required to be able to visit the cell `(row, col)`, which means you can visit the cell `(row, col)` only when the time you visit it is greater than or equal to `grid[row][col]`.

You are standing in the **top-left** cell of the matrix in the <code>0<sup>th</sup></code> second, and you must move to **any** adjacent cell in the four directions: up, down, left, and right. Each move you make takes 1 second.

Return *the **minimum** time required in which you can visit the bottom-right cell of the matrix*. If you cannot visit the bottom-right cell, then return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2023/02/14/yetgriddrawio-8.png)
<pre>
<strong>Input:</strong> grid = [[0,1,3,2],[5,1,2,5],[4,3,8,6]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> One of the paths that we can take is the following:
- at t = 0, we are on the cell (0,0).
- at t = 1, we move to the cell (0,1). It is possible because grid[0][1] <= 1.
- at t = 2, we move to the cell (1,1). It is possible because grid[1][1] <= 2.
- at t = 3, we move to the cell (1,2). It is possible because grid[1][2] <= 3.
- at t = 4, we move to the cell (1,1). It is possible because grid[1][1] <= 4.
- at t = 5, we move to the cell (1,2). It is possible because grid[1][2] <= 5.
- at t = 6, we move to the cell (1,3). It is possible because grid[1][3] <= 6.
- at t = 7, we move to the cell (2,3). It is possible because grid[2][3] <= 7.
The final time is 7. It can be shown that it is the minimum time possible.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2023/02/14/yetgriddrawio-9.png)
<pre>
<strong>Input:</strong> grid = [[0,2,4],[3,2,1],[1,0,4]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no path from the top left to the bottom-right cell.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 1000`
* <code>4 <= m * n <= 10<sup>5</sup></code>
* <code>0 <= grid[i][j] <= 10<sup>5</sup></code>
* `grid[0][0] == 0`

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = HashSet::from([(0, 0)]);
        let mut heap = BinaryHeap::from([Reverse((0, 0, 0))]);

        while let Some(Reverse((t, i, j))) = heap.pop() {
            if i == m - 1 && j == n - 1 {
                return t;
            }

            if i > 0 && !visited.contains(&(i - 1, j)) {
                visited.insert((i - 1, j));
                if t + 1 >= grid[i - 1][j] {
                    heap.push(Reverse((t + 1, i - 1, j)));
                } else if t % 2 != grid[i - 1][j] % 2 {
                    heap.push(Reverse((grid[i - 1][j], i - 1, j)));
                } else {
                    heap.push(Reverse((grid[i - 1][j] + 1, i - 1, j)));
                }
            }
            if i < m - 1 && !visited.contains(&(i + 1, j)) {
                visited.insert((i + 1, j));
                if t + 1 >= grid[i + 1][j] {
                    heap.push(Reverse((t + 1, i + 1, j)));
                } else if t % 2 != grid[i + 1][j] % 2 {
                    heap.push(Reverse((grid[i + 1][j], i + 1, j)));
                } else {
                    heap.push(Reverse((grid[i + 1][j] + 1, i + 1, j)));
                }
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                visited.insert((i, j - 1));
                if t + 1 >= grid[i][j - 1] {
                    heap.push(Reverse((t + 1, i, j - 1)));
                } else if t % 2 != grid[i][j - 1] % 2 {
                    heap.push(Reverse((grid[i][j - 1], i, j - 1)));
                } else {
                    heap.push(Reverse((grid[i][j - 1] + 1, i, j - 1)));
                }
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                visited.insert((i, j + 1));
                if t + 1 >= grid[i][j + 1] {
                    heap.push(Reverse((t + 1, i, j + 1)));
                } else if t % 2 != grid[i][j + 1] % 2 {
                    heap.push(Reverse((grid[i][j + 1], i, j + 1)));
                } else {
                    heap.push(Reverse((grid[i][j + 1] + 1, i, j + 1)));
                }
            }
        }

        unreachable!()
    }
}
```
