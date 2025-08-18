# 1914. 循环轮转矩阵
给你一个大小为 `m x n` 的整数矩阵 `grid`，其中 `m` 和 `n` 都是 **偶数** ；另给你一个整数 `k` 。

矩阵由若干层组成，如下图所示，每种颜色代表一层：

![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid.png)

矩阵的循环轮转是通过分别循环轮转矩阵中的每一层完成的。在对某一层进行一次循环旋转操作时，层中的每一个元素将会取代其 **逆时针** 方向的相邻元素。轮转示例如下：

![](https://assets.leetcode.com/uploads/2021/06/22/explanation_grid.jpg)

返回执行 `k` 次循环轮转操作后的矩阵。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/19/rod2.png)
<pre>
<strong>输入:</strong> grid = [[40,10],[30,20]], k = 1
<strong>输出:</strong> [[10,20],[40,30]]
<strong>解释:</strong> 上图展示了矩阵在执行循环轮转操作时每一步的状态。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid5.png)
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid6.png)
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid7.png)
<pre>
<strong>输入:</strong> grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]], k = 2
<strong>输出:</strong> [[3,4,8,12],[2,11,10,16],[1,7,6,15],[5,9,13,14]]
<strong>解释:</strong> 上图展示了矩阵在执行循环轮转操作时每一步的状态。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 50`
* `m` 和 `n` 都是 **偶数**
* `1 <= grid[i][j] <= 5000`
* <code>1 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m.min(n) / 2 {
            let mut indices = vec![];
            let mut vals = VecDeque::new();

            for r in i..m - i {
                indices.push([r, i]);
                vals.push_back(grid[r][i]);
            }
            for c in i + 1..n - i {
                indices.push([m - 1 - i, c]);
                vals.push_back(grid[m - 1 - i][c]);
            }
            for r in (i..m - 1 - i).rev() {
                indices.push([r, n - 1 - i]);
                vals.push_back(grid[r][n - 1 - i]);
            }
            for c in (i + 1..n - 1 - i).rev() {
                indices.push([i, c]);
                vals.push_back(grid[i][c]);
            }

            vals.rotate_right(k % vals.len());
            for j in 0..vals.len() {
                grid[indices[j][0]][indices[j][1]] = vals[j];
            }
        }

        grid
    }
}
```
