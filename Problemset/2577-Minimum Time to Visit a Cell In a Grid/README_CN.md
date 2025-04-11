# 2577. 在网格图中访问一个格子的最少时间
给你一个 `m x n` 的矩阵 `grid` ，每个元素都为 **非负** 整数，其中 `grid[row][col]` 表示可以访问格子 `(row, col)` 的 **最早** 时间。也就是说当你访问格子 `(row, col)` 时，最少已经经过的时间为 `grid[row][col]` 。

你从 **最左上角** 出发，出发时刻为 `0` ，你必须一直移动到上下左右相邻四个格子中的 **任意** 一个格子（即不能停留在格子上）。每次移动都需要花费 1 单位时间。

请你返回 **最早** 到达右下角格子的时间，如果你无法到达右下角的格子，请你返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/02/14/yetgriddrawio-8.png)
<pre>
<strong>输入:</strong> grid = [[0,1,3,2],[5,1,2,5],[4,3,8,6]]
<strong>输出:</strong> 7
<strong>解释:</strong> 一条可行的路径为：
- 时刻 t = 0 ，我们在格子 (0,0) 。
- 时刻 t = 1 ，我们移动到格子 (0,1) ，可以移动的原因是 grid[0][1] <= 1 。
- 时刻 t = 2 ，我们移动到格子 (1,1) ，可以移动的原因是 grid[1][1] <= 2 。
- 时刻 t = 3 ，我们移动到格子 (1,2) ，可以移动的原因是 grid[1][2] <= 3 。
- 时刻 t = 4 ，我们移动到格子 (1,1) ，可以移动的原因是 grid[1][1] <= 4 。
- 时刻 t = 5 ，我们移动到格子 (1,2) ，可以移动的原因是 grid[1][2] <= 5 。
- 时刻 t = 6 ，我们移动到格子 (1,3) ，可以移动的原因是 grid[1][3] <= 6 。
- 时刻 t = 7 ，我们移动到格子 (2,3) ，可以移动的原因是 grid[2][3] <= 7 。
最终到达时刻为 7 。这是最早可以到达的时间。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2023/02/14/yetgriddrawio-9.png)
<pre>
<strong>输入:</strong> grid = [[0,2,4],[3,2,1],[1,0,4]]
<strong>输出:</strong> -1
<strong>解释:</strong> 没法从左上角按题目规定走到右下角。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 1000`
* <code>4 <= m * n <= 10<sup>5</sup></code>
* <code>0 <= grid[i][j] <= 10<sup>5</sup></code>
* `grid[0][0] == 0`

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = HashSet::from([(0, 0)]);
        let mut heap = BinaryHeap::from([Reverse((0, 0, 0))]);

        while let Some(Reverse((t, i, j))) = heap.pop() {
            if i == m - 1 && j == n - 1 {
                return t;
            }

            if i > 0 && !visited.contains(&(i - 1, j)) {
                visited.insert((i - 1, j));
                if t + 1 >= grid[i - 1][j] {
                    heap.push(Reverse((t + 1, i - 1, j)));
                } else if t % 2 != grid[i - 1][j] % 2 {
                    heap.push(Reverse((grid[i - 1][j], i - 1, j)));
                } else {
                    heap.push(Reverse((grid[i - 1][j] + 1, i - 1, j)));
                }
            }
            if i < m - 1 && !visited.contains(&(i + 1, j)) {
                visited.insert((i + 1, j));
                if t + 1 >= grid[i + 1][j] {
                    heap.push(Reverse((t + 1, i + 1, j)));
                } else if t % 2 != grid[i + 1][j] % 2 {
                    heap.push(Reverse((grid[i + 1][j], i + 1, j)));
                } else {
                    heap.push(Reverse((grid[i + 1][j] + 1, i + 1, j)));
                }
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                visited.insert((i, j - 1));
                if t + 1 >= grid[i][j - 1] {
                    heap.push(Reverse((t + 1, i, j - 1)));
                } else if t % 2 != grid[i][j - 1] % 2 {
                    heap.push(Reverse((grid[i][j - 1], i, j - 1)));
                } else {
                    heap.push(Reverse((grid[i][j - 1] + 1, i, j - 1)));
                }
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                visited.insert((i, j + 1));
                if t + 1 >= grid[i][j + 1] {
                    heap.push(Reverse((t + 1, i, j + 1)));
                } else if t % 2 != grid[i][j + 1] % 2 {
                    heap.push(Reverse((grid[i][j + 1], i, j + 1)));
                } else {
                    heap.push(Reverse((grid[i][j + 1] + 1, i, j + 1)));
                }
            }
        }

        unreachable!()
    }
}
```
