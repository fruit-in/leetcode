# 1559. Detect Cycles in 2D Grid
Given a 2D array of characters `grid` of size `m x n`, you need to find if there exists any cycle consisting of the **same value** in `grid`.

A cycle is a path of **length 4 or more** in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the **same value** of the current cell.

Also, you cannot move to the cell that you visited in your last move. For example, the cycle `(1, 1) -> (1, 2) -> (1, 1)` is invalid because from `(1, 2)` we visited `(1, 1)` which was the last visited cell.

Return `true` if any cycle of the same value exists in `grid`, otherwise, return `false`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/15/1.png)
<pre>
<strong>Input:</strong> grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
<strong>Output:</strong> true
<strong>Explanation:</strong> There are two valid cycles shown in different colors in the image below:
<img src="https://assets.leetcode.com/uploads/2020/07/15/11.png">
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/07/15/22.png)
<pre>
<strong>Input:</strong> grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
<strong>Output:</strong> true
<strong>Explanation:</strong> There is only one valid cycle highlighted in the image below:
<img src="https://assets.leetcode.com/uploads/2020/07/15/2.png">
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/07/15/3.png)
<pre>
<strong>Input:</strong> grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
<strong>Output:</strong> false
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 500`
* `grid` consists only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited0 = HashSet::new();

        for r in 0..m {
            for c in 0..n {
                if visited0.contains(&(r, c)) {
                    continue;
                }

                let value = grid[r][c];
                let mut stack = vec![('\0', r, c)];
                let mut visited1 = HashSet::from([(r, c)]);

                while let Some((from, r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == value && from != 'U' {
                        if visited1.contains(&(r - 1, c)) {
                            return true;
                        }
                        stack.push(('D', r - 1, c));
                        visited1.insert((r - 1, c));
                    }
                    if r < m - 1 && grid[r + 1][c] == value && from != 'D' {
                        if visited1.contains(&(r + 1, c)) {
                            return true;
                        }
                        stack.push(('U', r + 1, c));
                        visited1.insert((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == value && from != 'L' {
                        if visited1.contains(&(r, c - 1)) {
                            return true;
                        }
                        stack.push(('R', r, c - 1));
                        visited1.insert((r, c - 1));
                    }
                    if c < n - 1 && grid[r][c + 1] == value && from != 'R' {
                        if visited1.contains(&(r, c + 1)) {
                            return true;
                        }
                        stack.push(('L', r, c + 1));
                        visited1.insert((r, c + 1));
                    }
                }

                visited0.extend(visited1);
            }
        }

        false
    }
}
```
