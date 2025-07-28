# 864. Shortest Path to Get All Keys
You are given an `m x n` grid `grid` where:
* `'.'` is an empty cell.
* `'#'` is a wall.
* `'@'` is the starting point.
* Lowercase letters represent keys.
* Uppercase letters represent locks.

You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.

If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.

For some `1 <= k <= 6`, there is exactly one lowercase and one uppercase letter of the first `k` letters of the English alphabet in the grid. This means that there is exactly one key for each lock, and one lock for each key; and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.

Return *the lowest number of moves to acquire all keys*. If it is impossible, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-keys2.jpg)
<pre>
<strong>Input:</strong> grid = ["@.a..","###.#","b.A.B"]
<strong>Output:</strong> 8
<strong>Explanation:</strong> Note that the goal is to obtain all the keys not to open all the locks.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-key2.jpg)
<pre>
<strong>Input:</strong> grid = ["@..aA","..B#.","....b"]
<strong>Output:</strong> 6
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-keys3.jpg)
<pre>
<strong>Input:</strong> grid = ["@Aa"]
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 30`
* `grid[i][j]` is either an English letter, `'.'`, `'#'`, or `'@'`.
* There is exactly one `'@'` in the grid.
* The number of keys in the grid is in the range `[1, 6]`.
* Each key in the grid is **unique**.
* Each key in the grid has a matching lock.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid = grid.iter().map(|row| row.as_bytes()).collect::<Vec<_>>();
        let (m, n) = (grid.len(), grid[0].len());
        let mut k = 0;
        let mut visited = HashSet::new();
        let mut deque = VecDeque::new();

        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == b'@' {
                    visited.insert((r, c, 0));
                    deque.push_back((r, c, 0, 0));
                } else if grid[r][c].is_ascii_lowercase() {
                    k += 1;
                }
            }
        }

        while let Some((r, c, keys, moves)) = deque.pop_front() {
            if keys == (1 << k) - 1 {
                return moves;
            }

            if r > 0
                && grid[r - 1][c] != b'#'
                && (!grid[r - 1][c].is_ascii_uppercase()
                    || (keys >> (grid[r - 1][c] - b'A')) & 1 == 1)
            {
                if grid[r - 1][c].is_ascii_lowercase()
                    && visited.insert((r - 1, c, keys | (1 << (grid[r - 1][c] - b'a'))))
                {
                    deque.push_back((r - 1, c, keys | (1 << (grid[r - 1][c] - b'a')), moves + 1));
                } else if !grid[r - 1][c].is_ascii_lowercase() && visited.insert((r - 1, c, keys)) {
                    deque.push_back((r - 1, c, keys, moves + 1));
                }
            }
            if r < m - 1
                && grid[r + 1][c] != b'#'
                && (!grid[r + 1][c].is_ascii_uppercase()
                    || (keys >> (grid[r + 1][c] - b'A')) & 1 == 1)
            {
                if grid[r + 1][c].is_ascii_lowercase()
                    && visited.insert((r + 1, c, keys | (1 << (grid[r + 1][c] - b'a'))))
                {
                    deque.push_back((r + 1, c, keys | (1 << (grid[r + 1][c] - b'a')), moves + 1));
                } else if !grid[r + 1][c].is_ascii_lowercase() && visited.insert((r + 1, c, keys)) {
                    deque.push_back((r + 1, c, keys, moves + 1));
                }
            }
            if c > 0
                && grid[r][c - 1] != b'#'
                && (!grid[r][c - 1].is_ascii_uppercase()
                    || (keys >> (grid[r][c - 1] - b'A')) & 1 == 1)
            {
                if grid[r][c - 1].is_ascii_lowercase()
                    && visited.insert((r, c - 1, keys | (1 << (grid[r][c - 1] - b'a'))))
                {
                    deque.push_back((r, c - 1, keys | (1 << (grid[r][c - 1] - b'a')), moves + 1));
                } else if !grid[r][c - 1].is_ascii_lowercase() && visited.insert((r, c - 1, keys)) {
                    deque.push_back((r, c - 1, keys, moves + 1));
                }
            }
            if c < n - 1
                && grid[r][c + 1] != b'#'
                && (!grid[r][c + 1].is_ascii_uppercase()
                    || (keys >> (grid[r][c + 1] - b'A')) & 1 == 1)
            {
                if grid[r][c + 1].is_ascii_lowercase()
                    && visited.insert((r, c + 1, keys | (1 << (grid[r][c + 1] - b'a'))))
                {
                    deque.push_back((r, c + 1, keys | (1 << (grid[r][c + 1] - b'a')), moves + 1));
                } else if !grid[r][c + 1].is_ascii_lowercase() && visited.insert((r, c + 1, keys)) {
                    deque.push_back((r, c + 1, keys, moves + 1));
                }
            }
        }

        -1
    }
}
```
