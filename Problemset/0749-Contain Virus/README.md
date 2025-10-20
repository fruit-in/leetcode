# 749. Contain Virus
A virus is spreading rapidly, and your task is to quarantine the infected area by installing walls.

The world is modeled as an `m x n` binary grid `isInfected`, where `isInfected[i][j] == 0` represents uninfected cells, and `isInfected[i][j] == 1` represents cells contaminated with the virus. A wall (and only one wall) can be installed between any two **4-directionally** adjacent cells, on the shared boundary.

Every night, the virus spreads to all neighboring cells in all four directions unless blocked by a wall. Resources are limited. Each day, you can install walls around only one region (i.e., the affected area (continuous block of infected cells) that threatens the most uninfected cells the following night). There **will never be a tie**.

Return *the number of walls used to quarantine all the infected regions*. If the world will become fully infected, return the number of walls used.

#### Example 1:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus11-grid.jpg">
<strong>Input:</strong> isInfected = [[0,1,0,0,0,0,0,1],[0,1,0,0,0,0,0,1],[0,0,0,0,0,0,0,1],[0,0,0,0,0,0,0,0]]
<strong>Output:</strong> 10
<strong>Explanation:</strong> There are 2 contaminated regions.
On the first day, add 5 walls to quarantine the viral region on the left. The board after the virus spreads is:
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus12edited-grid.jpg">
On the second day, add 5 walls to quarantine the viral region on the right. The virus is fully contained.
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus13edited-grid.jpg">
</pre>

#### Example 2:
<pre>
<img src="https://assets.leetcode.com/uploads/2021/06/01/virus2-grid.jpg">
<strong>Input:</strong> isInfected = [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Even though there is only one cell saved, there are 4 walls built.
Notice that walls are only built on the shared boundary of two different cells.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> isInfected = [[1,1,1,0,0,0,0,0,0],[1,0,1,0,1,1,1,1,1],[1,1,1,0,0,0,0,0,0]]
<strong>Output:</strong> 13
<strong>Explanation:</strong> The region on the left only builds two new walls.
</pre>

#### Constraints:
* `m == isInfected.length`
* `n == isInfected[i].length`
* `1 <= m, n <= 50`
* `isInfected[i][j]` is either `0` or `1`.
* There is always a contiguous viral region throughout the described process that will **infect strictly more uncontaminated squares** in the next round.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let m = is_infected.len();
        let n = is_infected[0].len();
        let mut walls = HashSet::new();

        loop {
            let mut visited = HashSet::new();
            let mut spreads = HashMap::new();
            let mut areas = HashMap::new();
            let mut needs = HashMap::new();
            let mut max_threat_level = 0;
            let mut max_threat_area = (0, 0);

            for i in 0..m {
                for j in 0..n {
                    if is_infected[i][j] != 1 || visited.contains(&(i, j)) {
                        continue;
                    }

                    let mut stack = vec![(i, j)];
                    visited.insert((i, j));
                    areas.insert((i, j), vec![(i, j)]);
                    spreads.insert((i, j), HashSet::new());
                    needs.insert((i, j), HashSet::new());

                    while let Some((r, c)) = stack.pop() {
                        if r > 0 {
                            if is_infected[r - 1][c] == 1 && !visited.contains(&(r - 1, c)) {
                                stack.push((r - 1, c));
                                visited.insert((r - 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r - 1, c));
                            } else if is_infected[r - 1][c] == 0
                                && !walls.contains(&(r - 1, c, 'D'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r - 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r - 1, c, 'D'));
                            }
                        }
                        if r < m - 1 {
                            if is_infected[r + 1][c] == 1 && !visited.contains(&(r + 1, c)) {
                                stack.push((r + 1, c));
                                visited.insert((r + 1, c));
                                areas.get_mut(&(i, j)).unwrap().push((r + 1, c));
                            } else if is_infected[r + 1][c] == 0 && !walls.contains(&(r, c, 'D')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r + 1, c));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'D'));
                            }
                        }
                        if c > 0 {
                            if is_infected[r][c - 1] == 1 && !visited.contains(&(r, c - 1)) {
                                stack.push((r, c - 1));
                                visited.insert((r, c - 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c - 1));
                            } else if is_infected[r][c - 1] == 0
                                && !walls.contains(&(r, c - 1, 'R'))
                            {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c - 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c - 1, 'R'));
                            }
                        }
                        if c < n - 1 {
                            if is_infected[r][c + 1] == 1 && !visited.contains(&(r, c + 1)) {
                                stack.push((r, c + 1));
                                visited.insert((r, c + 1));
                                areas.get_mut(&(i, j)).unwrap().push((r, c + 1));
                            } else if is_infected[r][c + 1] == 0 && !walls.contains(&(r, c, 'R')) {
                                spreads.get_mut(&(i, j)).unwrap().insert((r, c + 1));
                                needs.get_mut(&(i, j)).unwrap().insert((r, c, 'R'));
                            }
                        }
                    }

                    if spreads[&(i, j)].len() > max_threat_level {
                        max_threat_level = spreads[&(i, j)].len();
                        max_threat_area = (i, j);
                    }
                }
            }

            if max_threat_level == 0 {
                break;
            }

            for (&(r, c), s) in spreads.iter() {
                if (r, c) != max_threat_area {
                    for &(r, c) in s {
                        is_infected[r][c] = 1;
                    }
                }
            }

            for &(r, c) in &areas[&max_threat_area] {
                is_infected[r][c] = -1;
            }

            for &(r, c, d) in &needs[&max_threat_area] {
                walls.insert((r, c, d));
            }
        }

        walls.len() as i32
    }
}
```
