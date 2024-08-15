# 934. Shortest Bridge
You are given an `n x n` binary matrix `grid` where `1` represents land and `0` represents water.

An **island** is a 4-directionally connected group of `1`'s not connected to any other `1`'s. There are **exactly two islands** in `grid`.

You may change `0`'s to `1`'s to connect the two islands to form **one island**.

Return *the smallest number of* `0`*'s you must flip to connect the two islands*.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[0,1],[1,0]]
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[0,1,0],[0,0,0],[0,0,1]]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `2 <= n <= 100`
* `grid[i][j]` is either `0` or `1`.
* There are exactly two islands in `grid`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut island = HashSet::new();
        let mut edges = HashSet::new();
        let mut stack = vec![];
        let mut ret = i32::MAX;

        'outer: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    island.insert((i, j));
                    stack.push((i, j));
                    break 'outer;
                }
            }
        }

        while let Some((i, j)) = stack.pop() {
            let mut count = 0;

            if i > 0 && grid[i - 1][j] == 1 {
                if !island.contains(&(i - 1, j)) {
                    island.insert((i - 1, j));
                    stack.push((i - 1, j));
                }
                count += 1;
            }
            if i < n - 1 && grid[i + 1][j] == 1 {
                if !island.contains(&(i + 1, j)) {
                    island.insert((i + 1, j));
                    stack.push((i + 1, j));
                }
                count += 1;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                if !island.contains(&(i, j - 1)) {
                    island.insert((i, j - 1));
                    stack.push((i, j - 1));
                }
                count += 1;
            }
            if j < n - 1 && grid[i][j + 1] == 1 {
                if !island.contains(&(i, j + 1)) {
                    island.insert((i, j + 1));
                    stack.push((i, j + 1));
                }
                count += 1;
            }

            if count < 4 {
                edges.insert((i, j));
            }
        }

        for i0 in 0..n {
            for j0 in 0..n {
                if grid[i0][j0] == 0 || island.contains(&(i0, j0)) {
                    continue;
                }

                for &(i1, j1) in edges.iter() {
                    ret =
                        ret.min((i0 as i32 - i1 as i32).abs() + (j0 as i32 - j1 as i32).abs() - 1);
                }
            }
        }

        ret
    }
}
```
