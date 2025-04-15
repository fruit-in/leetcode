# 2258. Escape the Spreading Fire
You are given a **0-indexed** 2D integer array `grid` of size `m x n` which represents a field. Each cell has one of three values:
* `0` represents grass,
* `1` represents fire,
* `2` represents a wall that you and fire cannot pass through.

You are situated in the top-left cell, `(0, 0)`, and you want to travel to the safehouse at the bottom-right cell, `(m - 1, n - 1)`. Every minute, you may move to an **adjacent** grass cell. **After** your move, every fire cell will spread to all **adjacent** cells that are not walls.

Return *the **maximum** number of minutes that you can stay in your initial position before moving while still safely reaching the safehouse*. If this is impossible, return `-1`. If you can **always** reach the safehouse regardless of the minutes stayed, return <code>10<sup>9</sup></code>.

Note that even if the fire spreads to the safehouse immediately after you have reached it, it will be counted as safely reaching the safehouse.

A cell is **adjacent** to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/10/ex1new.jpg)
<pre>
<strong>Input:</strong> grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above shows the scenario where you stay in the initial position for 3 minutes.
You will still be able to safely reach the safehouse.
Staying for more than 3 minutes will not allow you to safely reach the safehouse.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/10/ex2new2.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,0,0],[0,1,2,0],[0,2,0,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The figure above shows the scenario where you immediately move towards the safehouse.
Fire will spread to any cell you move towards and it is impossible to safely reach the safehouse.
Thus, -1 is returned.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/03/10/ex3new.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,0],[2,2,0],[1,2,0]]
<strong>Output:</strong> 1000000000
<strong>Explanation:</strong> The figure above shows the initial grid.
Notice that the fire is contained by walls and you will always be able to safely reach the safehouse.
Thus, 10<sup>9</sup> is returned.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 300`
* <code>4 <= m * n <= 2 * 104</sup></code>
* `grid[i][j]` is either `0`, `1`, or `2`.
* `grid[0][0] == grid[m - 1][n - 1] == 0`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    fn canReach(
        mut grid: Vec<Vec<i32>>,
        mut fire: VecDeque<(usize, usize, i32)>,
        mut minutes: i32,
    ) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut person = VecDeque::from([(0, 0, minutes)]);
        grid[0][0] = -1;

        while let Some(&(i, j, t)) = fire.front() {
            if t >= minutes {
                break;
            }

            if i > 0 && grid[i - 1][j] < 1 {
                grid[i - 1][j] = 1;
                fire.push_back((i - 1, j, t + 1));
            }
            if i + 1 < m && grid[i + 1][j] < 1 {
                grid[i + 1][j] = 1;
                fire.push_back((i + 1, j, t + 1));
            }
            if j > 0 && grid[i][j - 1] < 1 {
                grid[i][j - 1] = 1;
                fire.push_back((i, j - 1, t + 1));
            }
            if j + 1 < n && grid[i][j + 1] < 1 {
                grid[i][j + 1] = 1;
                fire.push_back((i, j + 1, t + 1));
            }

            fire.pop_front();
        }

        while !person.is_empty() {
            while let Some(&(i, j, t)) = person.front() {
                if i == m - 1 && j == n - 1 {
                    return true;
                }

                if t > minutes {
                    break;
                }

                if grid[i][j] < 1 {
                    if i > 0 && grid[i - 1][j] == 0 {
                        grid[i - 1][j] = -1;
                        person.push_back((i - 1, j, t + 1));
                    }
                    if i + 1 < m && grid[i + 1][j] == 0 {
                        grid[i + 1][j] = -1;
                        person.push_back((i + 1, j, t + 1));
                    }
                    if j > 0 && grid[i][j - 1] == 0 {
                        grid[i][j - 1] = -1;
                        person.push_back((i, j - 1, t + 1));
                    }
                    if j + 1 < n && grid[i][j + 1] == 0 {
                        grid[i][j + 1] = -1;
                        person.push_back((i, j + 1, t + 1));
                    }
                }

                person.pop_front();
            }

            while let Some(&(i, j, t)) = fire.front() {
                if t > minutes {
                    break;
                }

                if i > 0 && grid[i - 1][j] < 1 {
                    grid[i - 1][j] = 1;
                    fire.push_back((i - 1, j, t + 1));
                }
                if i + 1 < m && grid[i + 1][j] < 1 {
                    grid[i + 1][j] = 1;
                    fire.push_back((i + 1, j, t + 1));
                }
                if j > 0 && grid[i][j - 1] < 1 {
                    grid[i][j - 1] = 1;
                    fire.push_back((i, j - 1, t + 1));
                }
                if j + 1 < n && grid[i][j + 1] < 1 {
                    grid[i][j + 1] = 1;
                    fire.push_back((i, j + 1, t + 1));
                }

                fire.pop_front();
            }

            minutes += 1;
        }

        false
    }
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut init_fire = VecDeque::new();
        let mut lo = 0;
        let mut hi = (m * n) as i32;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    init_fire.push_back((i, j, 0));
                }
            }
        }

        if !Self::canReach(grid.clone(), init_fire.clone(), lo) {
            return -1;
        }
        if Self::canReach(grid.clone(), init_fire.clone(), hi) {
            return 1_000_000_000;
        }

        while lo < hi {
            let mid = (lo + hi + 1) / 2;

            if Self::canReach(grid.clone(), init_fire.clone(), mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        hi
    }
}
```
