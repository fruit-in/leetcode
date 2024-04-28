# 778. Swim in Rising Water
You are given an `n x n` integer matrix `grid` where each value `grid[i][j]` represents the elevation at that point `(i, j)`.

The rain starts to fall. At time `t`, the depth of the water everywhere is `t`. You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most `t`. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.

Return *the least time until you can reach the bottom right square* `(n - 1, n - 1)` *if you start at the top left square* `(0, 0)`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/29/swim1-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[0,2],[1,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
At time 0, you are in grid location (0, 0).
You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
You cannot reach point (1, 1) until time 3.
When the depth of water is 3, we can swim anywhere inside the grid.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/29/swim2-grid-1.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
<strong>Output:</strong> 16
<strong>Explanation:</strong> The final route is shown.
We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
</pre>

#### Constraints:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 50`
* <code>0 <= grid[i][j] < n<sup>2</sup></code>
* Each value `grid[i][j]` is **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::new();

        while let Some((t, i, j)) = heap.pop() {
            if i == n - 1 && j == n - 1 {
                return -t;
            }

            if visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));

            if i > 0 && !visited.contains(&(i - 1, j)) {
                heap.push((t.min(-grid[i - 1][j]), i - 1, j));
            }
            if i < n - 1 && !visited.contains(&(i + 1, j)) {
                heap.push((t.min(-grid[i + 1][j]), i + 1, j));
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                heap.push((t.min(-grid[i][j - 1]), i, j - 1));
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                heap.push((t.min(-grid[i][j + 1]), i, j + 1));
            }
        }

        unreachable!()
    }
}
```
