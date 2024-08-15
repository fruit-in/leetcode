# 934. 最短的桥
给你一个大小为 `n x n` 的二元矩阵 `grid` ，其中 `1` 表示陆地，`0` 表示水域。

**岛** 是由四面相连的 `1` 形成的一个最大组，即不会与非组内的任何其他 `1` 相连。`grid` 中 **恰好存在两座岛** 。

你可以将任意数量的 `0` 变为 `1` ，以使两座岛连接起来，变成 **一座岛** 。

返回必须翻转的 `0` 的最小数目。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[0,1],[1,0]]
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[0,1,0],[0,0,0],[0,0,1]]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `n == grid.length == grid[i].length`
* `2 <= n <= 100`
* `grid[i][j]` 为 `0` 或 `1`
* `grid` 中恰有两个岛

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut island = HashSet::new();
        let mut edges = HashSet::new();
        let mut stack = vec![];
        let mut ret = i32::MAX;

        'outer: for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    island.insert((i, j));
                    stack.push((i, j));
                    break 'outer;
                }
            }
        }

        while let Some((i, j)) = stack.pop() {
            let mut count = 0;

            if i > 0 && grid[i - 1][j] == 1 {
                if !island.contains(&(i - 1, j)) {
                    island.insert((i - 1, j));
                    stack.push((i - 1, j));
                }
                count += 1;
            }
            if i < n - 1 && grid[i + 1][j] == 1 {
                if !island.contains(&(i + 1, j)) {
                    island.insert((i + 1, j));
                    stack.push((i + 1, j));
                }
                count += 1;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                if !island.contains(&(i, j - 1)) {
                    island.insert((i, j - 1));
                    stack.push((i, j - 1));
                }
                count += 1;
            }
            if j < n - 1 && grid[i][j + 1] == 1 {
                if !island.contains(&(i, j + 1)) {
                    island.insert((i, j + 1));
                    stack.push((i, j + 1));
                }
                count += 1;
            }

            if count < 4 {
                edges.insert((i, j));
            }
        }

        for i0 in 0..n {
            for j0 in 0..n {
                if grid[i0][j0] == 0 || island.contains(&(i0, j0)) {
                    continue;
                }

                for &(i1, j1) in edges.iter() {
                    ret =
                        ret.min((i0 as i32 - i1 as i32).abs() + (j0 as i32 - j1 as i32).abs() - 1);
                }
            }
        }

        ret
    }
}
```
