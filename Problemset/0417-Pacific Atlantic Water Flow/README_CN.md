# 417. 太平洋大西洋水流问题
有一个 `m × n` 的矩形岛屿，与 **太平洋** 和 **大西洋** 相邻。 **“太平洋”** 处于大陆的左边界和上边界，而 **“大西洋”** 处于大陆的右边界和下边界。

这个岛被分割成一个由若干方形单元格组成的网格。给定一个 `m x n` 的整数矩阵 `heights` ， `heights[r][c]` 表示坐标 `(r, c)` 上单元格 **高于海平面的高度** 。

岛上雨水较多，如果相邻单元格的高度 **小于或等于** 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。水可以从海洋附近的任何单元格流入海洋。

返回网格坐标 `result` 的 **2D 列表** ，其中 <code>result[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> 表示雨水从单元格 <code>(r<sub>i</sub>, c<sub>i</sub>)</code> 流动 **既可流向太平洋也可流向大西洋** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg)
<pre>
<strong>输入:</strong> heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
<strong>输出:</strong> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> heights = [[1]]
<strong>输出:</strong> [[0,0]]
</pre>

#### 提示:
* `m == heights.length`
* `n == heights[r].length`
* `1 <= m, n <= 200`
* <code>0 <= heights[r][c] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut cells = vec![];
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        for r in 0..m {
            cells.push((r, 0));
            cells.push((r, n - 1));
            pacific.insert((r, 0));
            atlantic.insert((r, n - 1));
        }
        for c in 0..n {
            cells.push((0, c));
            cells.push((m - 1, c));
            pacific.insert((0, c));
            atlantic.insert((m - 1, c));
        }

        while let Some((r0, c0)) = cells.pop() {
            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let r1 = ((r0 as i32 + x).max(0) as usize).min(m - 1);
                let c1 = ((c0 as i32 + y).max(0) as usize).min(n - 1);

                if heights[r0][c0] > heights[r1][c1] {
                    continue;
                }
                if pacific.contains(&(r0, c0)) && pacific.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
                if atlantic.contains(&(r0, c0)) && atlantic.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
            }
        }

        pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect()
    }
}
```
