# 827. 最大人工岛
给你一个大小为 `n x n` 二进制矩阵 `grid` 。**最多** 只能将一格 `0` 变成 `1` 。

返回执行此操作后，`grid` 中最大的岛屿面积是多少？

**岛屿** 由一组上、下、左、右四个方向相连的 `1` 形成。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[1,0],[0,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 将一格0变成1，最终连通两个小岛得到面积为 3 的岛屿。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,1],[1,0]]
<strong>输出:</strong> 4
<strong>解释:</strong> 将一格0变成1，岛屿的面积扩大为 4。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1],[1,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 没有0可以让我们变成1，面积依然为 4。
</pre>

#### 提示:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 500`
* `grid[i][j]` 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let n = grid.len();
        let mut parent = HashMap::new();
        let mut count = HashMap::new();
        let mut ret = 0;

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 || parent.contains_key(&(r, c)) {
                    continue;
                }

                let mut stack = vec![(r, c)];
                parent.insert((r, c), (r, c));
                count.insert((r, c), 0);

                while let Some((i, j)) = stack.pop() {
                    *count.get_mut(&(r, c)).unwrap() += 1;

                    if i > 0 && grid[i - 1][j] == 1 && !parent.contains_key(&(i - 1, j)) {
                        stack.push((i - 1, j));
                        parent.insert((i - 1, j), (r, c));
                    }
                    if i < n - 1 && grid[i + 1][j] == 1 && !parent.contains_key(&(i + 1, j)) {
                        stack.push((i + 1, j));
                        parent.insert((i + 1, j), (r, c));
                    }
                    if j > 0 && grid[i][j - 1] == 1 && !parent.contains_key(&(i, j - 1)) {
                        stack.push((i, j - 1));
                        parent.insert((i, j - 1), (r, c));
                    }
                    if j < n - 1 && grid[i][j + 1] == 1 && !parent.contains_key(&(i, j + 1)) {
                        stack.push((i, j + 1));
                        parent.insert((i, j + 1), (r, c));
                    }
                }

                ret = ret.max(count[&(r, c)]);
            }
        }

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] == 0 {
                    let mut islands = HashSet::new();

                    if r > 0 && grid[r - 1][c] == 1 {
                        islands.insert(parent[&(r - 1, c)]);
                    }
                    if r < n - 1 && grid[r + 1][c] == 1 {
                        islands.insert(parent[&(r + 1, c)]);
                    }
                    if c > 0 && grid[r][c - 1] == 1 {
                        islands.insert(parent[&(r, c - 1)]);
                    }
                    if c < n - 1 && grid[r][c + 1] == 1 {
                        islands.insert(parent[&(r, c + 1)]);
                    }

                    ret = ret.max(1 + islands.iter().map(|&(r, c)| count[&(r, c)]).sum::<i32>());
                }
            }
        }

        ret
    }
}
```
