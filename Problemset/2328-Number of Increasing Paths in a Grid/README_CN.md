# 2328. 网格图中递增路径的数目
给你一个 `m x n` 的整数网格图 `grid` ，你可以从一个格子移动到 `4` 个方向相邻的任意一个格子。

请你返回在网格图中从 **任意** 格子出发，达到 **任意** 格子，且路径中的数字是 **严格递增** 的路径数目。由于答案可能会很大，请将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

如果两条路径中访问过的格子不是完全相同的，那么它们视为两条不同的路径。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/05/10/griddrawio-4.png)
<pre>
<strong>输入:</strong> grid = [[1,1],[3,4]]
<strong>输出:</strong> 8
<strong>解释:</strong> 严格递增路径包括：
- 长度为 1 的路径：[1]，[1]，[3]，[4] 。
- 长度为 2 的路径：[1 -> 3]，[1 -> 4]，[3 -> 4] 。
- 长度为 3 的路径：[1 -> 3 -> 4] 。
路径数目为 4 + 3 + 1 = 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1],[2]]
<strong>输出:</strong> 3
<strong>解释:</strong> 严格递增路径包括：
- 长度为 1 的路径：[1]，[2] 。
- 长度为 2 的路径：[1 -> 2] 。
路径数目为 2 + 1 = 3 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 1000`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= grid[i][j] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut indices = vec![];
        let mut paths_count = vec![vec![1; n]; m];
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                indices.push((i, j));
            }
        }

        indices.sort_unstable_by_key(|&(i, j)| grid[i][j]);

        for (i, j) in indices {
            if i > 0 && grid[i - 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i - 1][j]) % 1_000_000_007;
            }
            if i < m - 1 && grid[i + 1][j] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i + 1][j]) % 1_000_000_007;
            }
            if j > 0 && grid[i][j - 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j - 1]) % 1_000_000_007;
            }
            if j < n - 1 && grid[i][j + 1] < grid[i][j] {
                paths_count[i][j] = (paths_count[i][j] + paths_count[i][j + 1]) % 1_000_000_007;
            }

            ret = (ret + paths_count[i][j]) % 1_000_000_007;
        }

        ret
    }
}
```
