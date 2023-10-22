# 1368. 使网格图至少有一条有效路径的最小代价
给你一个 m x n 的网格图 `grid` 。 `grid` 中每个格子都有一个数字，对应着从该格子出发下一步走的方向。 `grid[i][j]` 中的数字可能为以下几种情况：

* **1** ，下一步往右走，也就是你会从 `grid[i][j]` 走到 `grid[i][j + 1]`
* **2** ，下一步往左走，也就是你会从 `grid[i][j]` 走到 `grid[i][j - 1]`
* **3** ，下一步往下走，也就是你会从 `grid[i][j]` 走到 `grid[i + 1][j]`
* **4** ，下一步往上走，也就是你会从 `grid[i][j]` 走到 `grid[i - 1][j]`

注意网格图中可能会有 **无效数字** ，因为它们可能指向 `grid` 以外的区域。

一开始，你会从最左上角的格子 `(0,0)` 出发。我们定义一条 **有效路径** 为从格子 `(0,0)` 出发，每一步都顺着数字对应方向走，最终在最右下角的格子 `(m - 1, n - 1)` 结束的路径。有效路径 **不需要是最短路径** 。

你可以花费 `cost = 1` 的代价修改一个格子中的数字，但每个格子中的数字 **只能修改一次** 。

请你返回让网格图至少有一条有效路径的最小代价。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/02/13/grid1.png)
<pre>
<strong>输入:</strong> grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
<strong>输出:</strong> 3
<strong>解释:</strong> You will start at point (0, 0).
The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
The total cost = 3.
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/02/13/grid2.png)
<pre>
<strong>输入:</strong> grid = [[1,1,3],[3,2,2],[1,1,4]]
<strong>输出:</strong> 0
<strong>解释:</strong> You can follow the path from (0, 0) to (2, 2).
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/02/13/grid3.png)
<pre>
<strong>输入:</strong> grid = [[1,2],[4,3]]
<strong>输出:</strong> 1

#### 示例 4:
<pre>
<strong>输入:</strong> grid = [[2,2,2],[2,2,2]]
<strong>输出:</strong> 3

#### 示例 5:
<pre>
<strong>输入:</strong> grid = [[4]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let m = grid.len();
        let n = grid[0].len();
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::from([(0, 0, 0)]);

        while let Some((cost, i0, j0)) = heap.pop() {
            if i0 < 0 || j0 < 0 || i0 >= m || j0 >= n {
                continue;
            }

            if i0 == m - 1 && j0 == n - 1 {
                return -cost;
            }

            seen.insert((i0, j0));

            for (i1, j1, g) in [
                (i0, j0 + 1, 1),
                (i0, j0 - 1, 2),
                (i0 + 1, j0, 3),
                (i0 - 1, j0, 4),
            ] {
                if !seen.contains(&(i1, j1)) {
                    if g == grid[i0 as usize][j0 as usize] {
                        heap.push((cost, i1, j1));
                    } else {
                        heap.push((cost - 1, i1, j1));
                    }
                }
            }
        }

        unreachable!()
    }
}
```
