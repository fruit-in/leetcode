# 1368. Minimum Cost to Make at Least One Valid Path in a Grid
Given an `m x n` grid. Each cell of the grid has a sign pointing to the next cell you should visit if you are currently in this cell. The sign of `grid[i][j]` can be:

* `1` which means go to the cell to the right. (i.e go from `grid[i][j]` to `grid[i][j + 1]`)
* `2` which means go to the cell to the left. (i.e go from `grid[i][j]` to `grid[i][j - 1]`)
* `3` which means go to the lower cell. (i.e go from `grid[i][j]` to `grid[i + 1][j]`)
* `4` which means go to the upper cell. (i.e go from `grid[i][j]` to `grid[i - 1][j]`)

Notice that there could be some signs on the cells of the grid that point outside the grid.

You will initially start at the upper left cell `(0, 0)`. A valid path in the grid is a path that starts from the upper left cell `(0, 0)` and ends at the bottom-right cell `(m - 1, n - 1)` following the signs on the grid. The valid path does not have to be the shortest.

You can modify the sign on a cell with `cost = 1`. You can modify the sign on a cell **one time only**.

Return *the minimum cost to make the grid have at least one valid path*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/13/grid1.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You will start at point (0, 0).
The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
The total cost = 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/02/13/grid2.png)
<pre>
<strong>Input:</strong> grid = [[1,1,3],[3,2,2],[1,1,4]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> You can follow the path from (0, 0) to (2, 2).
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/02/13/grid3.png)
<pre>
<strong>Input:</strong> grid = [[1,2],[4,3]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* `1 <= grid[i][j] <= 4`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let m = grid.len();
        let n = grid[0].len();
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::from([(0, 0, 0)]);

        while let Some((cost, i0, j0)) = heap.pop() {
            if i0 < 0 || j0 < 0 || i0 >= m || j0 >= n {
                continue;
            }

            if i0 == m - 1 && j0 == n - 1 {
                return -cost;
            }

            seen.insert((i0, j0));

            for (i1, j1, g) in [
                (i0, j0 + 1, 1),
                (i0, j0 - 1, 2),
                (i0 + 1, j0, 3),
                (i0 - 1, j0, 4),
            ] {
                if !seen.contains(&(i1, j1)) {
                    if g == grid[i0 as usize][j0 as usize] {
                        heap.push((cost, i1, j1));
                    } else {
                        heap.push((cost - 1, i1, j1));
                    }
                }
            }
        }

        unreachable!()
    }
}
```
