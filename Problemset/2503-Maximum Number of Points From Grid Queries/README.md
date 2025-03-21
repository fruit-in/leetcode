# 2503. Maximum Number of Points From Grid Queries
You are given an `m x n` integer matrix `grid` and an array `queries` of size `k`.

Find an array `answer` of size `k` such that for each integer `queries[i]` you start in the **top left** cell of the matrix and repeat the following process:
* If `queries[i]` is **strictly** greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any **adjacent** cell in all `4` directions: up, down, left, and right.
* Otherwise, you do not get any points, and you end this process.

After the process, `answer[i]` is the **maximum** number of points you can get. **Note** that for each query you are allowed to visit the same cell **multiple** times.

Return *the resulting array* `answer`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2025/03/15/image1.png)
<pre>
<strong>Input:</strong> grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
<strong>Output:</strong> [5,8,1]
<strong>Explanation:</strong> The diagrams above show which cells we visit to get points for each query.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/10/20/yetgriddrawio-2.png)
<pre>
<strong>Input:</strong> grid = [[5,2,1],[1,1,2]], queries = [3]
<strong>Output:</strong> [0]
<strong>Explanation:</strong> We can not get any points because the value of the top left cell is already greater than or equal to 3.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 1000`
* <code>4 <= m * n <= 10<sup>5</sup></code>
* `k == queries.length`
* <code>1 <= k <= 10<sup>4</sup></code>
* <code>1 <= grid[i][j], queries[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let (m, n) = (grid.len(), grid[0].len());
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::from([(0, 0)]);
        let mut indices = (0..queries.len()).collect::<Vec<_>>();
        let mut points = 0;
        let mut answer = vec![0; queries.len()];

        indices.sort_unstable_by_key(|&i| queries[i]);

        for &i in &indices {
            while queries[i] > -heap.peek().unwrap_or(&(-1000000, 0, 0)).0 {
                let (_, i, j) = heap.pop().unwrap();

                if i > 0 && !visited.contains(&(i - 1, j)) {
                    heap.push((-grid[i - 1][j], i - 1, j));
                    visited.insert((i - 1, j));
                }
                if i < m - 1 && !visited.contains(&(i + 1, j)) {
                    heap.push((-grid[i + 1][j], i + 1, j));
                    visited.insert((i + 1, j));
                }
                if j > 0 && !visited.contains(&(i, j - 1)) {
                    heap.push((-grid[i][j - 1], i, j - 1));
                    visited.insert((i, j - 1));
                }
                if j < n - 1 && !visited.contains(&(i, j + 1)) {
                    heap.push((-grid[i][j + 1], i, j + 1));
                    visited.insert((i, j + 1));
                }

                points += 1;
            }

            answer[i] = points;
        }

        answer
    }
}
```
