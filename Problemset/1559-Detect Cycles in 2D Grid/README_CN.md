# 1559. 二维网格图中探测环
给你一个二维字符网格数组 `grid` ，大小为 `m x n` ，你需要检查 `grid` 中是否存在 **相同值** 形成的环。

一个环是一条开始和结束于同一个格子的长度 **大于等于 4** 的路径。对于一个给定的格子，你可以移动到它上、下、左、右四个方向相邻的格子之一，可以移动的前提是这两个格子有 **相同的值** 。

同时，你也不能回到上一次移动时所在的格子。比方说，环  `(1, 1) -> (1, 2) -> (1, 1)` 是不合法的，因为从 `(1, 2)` 移动到 `(1, 1)` 回到了上一次移动时的格子。

如果 `grid` 中有相同值形成的环，请你返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/07/15/1.png)
<pre>
<strong>输入:</strong> grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
<strong>输出:</strong> true
<strong>解释:</strong> 如下图所示，有 2 个用不同颜色标出来的环：
<img src="https://assets.leetcode.com/uploads/2020/07/15/11.png">
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/07/15/22.png)
<pre>
<strong>输入:</strong> grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
<strong>输出:</strong> true
<strong>解释:</strong> 如下图所示，只有高亮所示的一个合法环：
<img src="https://assets.leetcode.com/uploads/2020/07/15/2.png">
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/07/15/3.png)
<pre>
<strong>输入:</strong> grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
<strong>输出:</strong> false
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m <= 500`
* `1 <= n <= 500`
* `grid` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
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
