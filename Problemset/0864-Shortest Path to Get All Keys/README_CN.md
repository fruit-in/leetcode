# 864. 获取所有钥匙的最短路径
给定一个二维网格 `grid` ，其中：
* `'.'` 代表一个空房间
* `'#'` 代表一堵墙
* `'@'` 是起点
* 小写字母代表钥匙
* 大写字母代表锁

我们从起点开始出发，一次移动是指向四个基本方向之一行走一个单位空间。我们不能在网格外面行走，也无法穿过一堵墙。如果途经一个钥匙，我们就把它捡起来。除非我们手里有对应的钥匙，否则无法通过锁。

假设 k 为 钥匙/锁 的个数，且满足 `1 <= k <= 6`，字母表中的前 `k` 个字母在网格中都有自己对应的一个小写和一个大写字母。换言之，每个锁有唯一对应的钥匙，每个钥匙也有唯一对应的锁。另外，代表钥匙和锁的字母互为大小写并按字母顺序排列。

返回获取所有钥匙所需要的移动的最少次数。如果无法获取所有钥匙，返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-keys2.jpg)
<pre>
<strong>输入:</strong> grid = ["@.a..","###.#","b.A.B"]
<strong>输出:</strong> 8
<strong>解释:</strong> 目标是获得所有钥匙，而不是打开所有锁。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-key2.jpg)
<pre>
<strong>输入:</strong> grid = ["@..aA","..B#.","....b"]
<strong>输出:</strong> 6
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-keys3.jpg)
<pre>
<strong>输入:</strong> grid = ["@Aa"]
<strong>输出:</strong> -1
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 30`
* `grid[i][j]` 只含有 `'.'`, `'#'`, `'@'`, `'a'-'f'` 以及 `'A'-'F'`
* 钥匙的数目范围是 `[1, 6]`
* 每个钥匙都对应一个 **不同** 的字母
* 每个钥匙正好打开一个对应的锁

## 题解 (Rust)

### 1. 题解
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
