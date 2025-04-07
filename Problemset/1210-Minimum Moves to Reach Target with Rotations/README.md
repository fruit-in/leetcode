# 1210. Minimum Moves to Reach Target with Rotations
In an `n*n` grid, there is a snake that spans 2 cells and starts moving from the top left corner at `(0, 0)` and `(0, 1)`. The grid has empty cells represented by zeros and blocked cells represented by ones. The snake wants to reach the lower right corner at `(n-1, n-2)` and `(n-1, n-1)`.

In one move the snake can:
* Move one cell to the right if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
* Move down one cell if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.
* Rotate clockwise if it's in a horizontal position and the two cells under it are both empty. In that case the snake moves from `(r, c)` and `(r, c+1)` to `(r, c)` and `(r+1, c)`.

![](https://assets.leetcode.com/uploads/2019/09/24/image-2.png)

* Rotate counterclockwise if it's in a vertical position and the two cells to its right are both empty. In that case the snake moves from `(r, c)` and `(r+1, c)` to `(r, c)` and `(r, c+1)`.

![](https://assets.leetcode.com/uploads/2019/09/24/image-1.png)

Return the minimum number of moves to reach the target.

If there is no way to reach the target, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/24/image.png)
<pre>
<strong>Input:</strong> grid = [[0,0,0,0,0,1],
               [1,1,0,0,1,0],
               [0,0,0,0,1,1],
               [0,0,1,0,1,0],
               [0,1,1,0,0,0],
               [0,1,1,0,0,0]]
<strong>Output:</strong> 11
<strong>Explanation:</strong>
One possible solution is [right, right, rotate clockwise, right, down, down, down, down, rotate counterclockwise, right, down].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[0,0,1,1,1,1],
               [0,0,0,0,1,1],
               [1,1,0,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,0]]
<strong>Output:</strong> 9
</pre>

#### Constraints:
* `2 <= n <= 100`
* `0 <= grid[i][j] <= 1`
* It is guaranteed that the snake starts at empty cells.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut deque = VecDeque::from([(0, 0, true, 0)]);
        let mut visited = HashSet::from([(0, 0, true)]);

        while let Some((r, c, horizontal, step)) = deque.pop_front() {
            if r == n - 1 && c == n - 2 && horizontal {
                return step;
            }

            if horizontal {
                if c < n - 2 && grid[r][c + 2] == 0 && !visited.contains(&(r, c + 1, true)) {
                    deque.push_back((r, c + 1, true, step + 1));
                    visited.insert((r, c + 1, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r + 1, c, true))
                {
                    deque.push_back((r + 1, c, true, step + 1));
                    visited.insert((r + 1, c, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, false))
                {
                    deque.push_back((r, c, false, step + 1));
                    visited.insert((r, c, false));
                }
            } else {
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c + 1, false))
                {
                    deque.push_back((r, c + 1, false, step + 1));
                    visited.insert((r, c + 1, false));
                }
                if r < n - 2 && grid[r + 2][c] == 0 && !visited.contains(&(r + 1, c, false)) {
                    deque.push_back((r + 1, c, false, step + 1));
                    visited.insert((r + 1, c, false));
                }
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, true))
                {
                    deque.push_back((r, c, true, step + 1));
                    visited.insert((r, c, true));
                }
            }
        }

        -1
    }
}
```
