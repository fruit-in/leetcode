# 2267. 检查是否有合法括号字符串路径
一个括号字符串是一个 **非空** 且只包含 `'('` 和 `')'` 的字符串。如果下面 **任意** 条件为 **真** ，那么这个括号字符串就是 **合法的** 。

* 字符串是 `()` 。
* 字符串可以表示为 `AB`（`A` 连接 `B`），`A` 和 `B` 都是合法括号序列。
* 字符串可以表示为 `(A)` ，其中 `A` 是合法括号序列。

给你一个 `m x n` 的括号网格图矩阵 `grid` 。网格图中一个 **合法括号路径** 是满足以下所有条件的一条路径：
* 路径开始于左上角格子 `(0, 0)` 。
* 路径结束于右下角格子 `(m - 1, n - 1)` 。
* 路径每次只会向 **下** 或者向 **右** 移动。
* 路径经过的格子组成的括号字符串是 **合法** 的。

如果网格图中存在一条 **合法括号路径** ，请返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/15/example1drawio.png)
<pre>
<strong>输入:</strong> grid = [["(","(","("],[")","(",")"],["(","(",")"],["(","(",")"]]
<strong>输出:</strong> true
<strong>解释:</strong> 上图展示了两条路径，它们都是合法括号字符串路径。
第一条路径得到的合法字符串是 "()(())" 。
第二条路径得到的合法字符串是 "((()))" 。
注意可能有其他的合法括号字符串路径。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/15/example2drawio.png)
<pre>
<strong>输入:</strong> grid = [[")",")"],["(","("]]
<strong>输出:</strong> false
<strong>解释:</strong> 两条可行路径分别得到 "))(" 和 ")((" 。由于它们都不是合法括号字符串，我们返回 false 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* `grid[i][j]` 要么是 `'('` ，要么是 `')'` 。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![(0, 0, 1)];
        let mut visited = HashSet::from([(0, 0, 1)]);

        if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' || (m + n) % 2 == 0 {
            return false;
        }

        while let Some((i, j, diff)) = stack.pop() {
            if i == m - 1 && j == n - 1 && diff == 0 {
                return true;
            }

            if i + 1 < m {
                let new_diff = diff + 81 - grid[i + 1][j] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i + 1, j, new_diff)) {
                    stack.push((i + 1, j, new_diff));
                    visited.insert((i + 1, j, new_diff));
                }
            }

            if j + 1 < n {
                let new_diff = diff + 81 - grid[i][j + 1] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i, j + 1, new_diff)) {
                    stack.push((i, j + 1, new_diff));
                    visited.insert((i, j + 1, new_diff));
                }
            }
        }

        false
    }
}
```
