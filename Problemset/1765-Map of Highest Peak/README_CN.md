# 1765. 地图中的最高点
给你一个大小为 `m x n` 的整数矩阵 `isWater` ，它代表了一个由 **陆地** 和 **水域** 单元格组成的地图。

* 如果 `isWater[i][j] == 0` ，格子 `(i, j)` 是一个 **陆地** 格子。
* 如果 `isWater[i][j] == 1` ，格子 `(i, j)` 是一个 **水域** 格子。

你需要按照如下规则给每个单元格安排高度：

* 每个格子的高度都必须是非负的。
* 如果一个格子是 **水域** ，那么它的高度必须为 `0` 。
* 任意相邻的格子高度差 **至多** 为 `1` 。当两个格子在正东、南、西、北方向上相互紧挨着，就称它们为相邻的格子。（也就是说它们有一条公共边）

找到一种安排高度的方案，使得矩阵中的最高高度值 **最大** 。

请你返回一个大小为 `m x n` 的整数矩阵 `height` ，其中 `height[i][j]` 是格子 `(i, j)` 的高度。如果有多种解法，请返回 **任意一个** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82045-am.png)
<pre>
<strong>输入:</strong> isWater = [[0,1],[0,0]]
<strong>输出:</strong> [[1,0],[2,1]]
<strong>解释:</strong> 上图展示了给各个格子安排的高度。
蓝色格子是水域格，绿色格子是陆地格。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82050-am.png)
<pre>
<strong>输入:</strong> isWater = [[0,0,1],[1,0,0],[0,0,0]]
<strong>输出:</strong> [[1,1,0],[0,1,1],[1,2,2]]
<strong>解释:</strong> 所有安排方案中，最高可行高度为 2 。
任意安排方案中，只要最高高度为 2 且符合上述规则的，都为可行方案。
</pre>

#### 提示:
* `m == isWater.length`
* `n == isWater[i].length`
* `1 <= m, n <= 1000`
* `isWater[i][j]` 要么是 `0` ，要么是 `1` 。
* 至少有 **1** 个水域格子。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let m = is_water.len();
        let n = is_water[0].len();
        let mut deque = VecDeque::new();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    deque.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = deque.pop_front() {
            if i > 0 && is_water[i - 1][j] == 0 {
                is_water[i - 1][j] = 1;
                deque.push_back((i - 1, j));
                ret[i - 1][j] = ret[i][j] + 1;
            }
            if i + 1 < m && is_water[i + 1][j] == 0 {
                is_water[i + 1][j] = 1;
                deque.push_back((i + 1, j));
                ret[i + 1][j] = ret[i][j] + 1;
            }
            if j > 0 && is_water[i][j - 1] == 0 {
                is_water[i][j - 1] = 1;
                deque.push_back((i, j - 1));
                ret[i][j - 1] = ret[i][j] + 1;
            }
            if j + 1 < n && is_water[i][j + 1] == 0 {
                is_water[i][j + 1] = 1;
                deque.push_back((i, j + 1));
                ret[i][j + 1] = ret[i][j] + 1;
            }
        }

        ret
    }
}
```
