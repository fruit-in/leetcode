# 1162. As Far from Land as Possible
Given an `n x n` `grid` containing only values `0` and `1`, where `0` represents water and `1` represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance. If no land or water exists in the grid, return `-1`.

The distance used in this problem is the Manhattan distance: the distance between two cells `(x0, y0)` and `(x1, y1)` is `|x0 - x1| + |y0 - y1|`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/05/03/1336_ex1.JPG)
<pre>
<strong>Input:</strong> grid = [[1,0,1],[0,0,0],[1,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The cell (1, 1) is as far as possible from all the land with distance 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/05/03/1336_ex2.JPG)
<pre>
<strong>Input:</strong> grid = [[1,0,0],[0,0,0],[0,0,0]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The cell (2, 2) is as far as possible from all the land with distance 4.
</pre>

#### Constraints:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 100`
* `grid[i][j]` is `0` or `1`

## Solutions (Rust)

### 1. BFS
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut ret = -1;

        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1 {
                    continue;
                }

                let mut cells = vec![(i, j, 0)].into_iter().collect::<VecDeque<_>>();
                let mut seen = vec![(i, j)].into_iter().collect::<HashSet<_>>();
                while let Some((x, y, d)) = cells.pop_front() {
                    if grid[x as usize][y as usize] == 1 {
                        ret = ret.max(d);
                        break;
                    }
                    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let x = x + dx;
                        let y = y + dy;
                        if x >= 0 && x < n && y >= 0 && y < n && !seen.contains(&(x, y)) {
                            cells.push_back((x, y, d + 1));
                            seen.insert((x, y));
                        }
                    }
                }
            }
        }

        ret
    }
}
```
