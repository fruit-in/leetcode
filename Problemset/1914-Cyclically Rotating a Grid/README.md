# 1914. Cyclically Rotating a Grid
You are given an `m x n` integer matrix `grid`, where `m` and `n` are both **even** integers, and an integer `k`.

The matrix is composed of several layers, which is shown in the below image, where each color is its own layer:

![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid.png)

A cyclic rotation of the matrix is done by cyclically rotating **each layer** in the matrix. To cyclically rotate a layer once, each element in the layer will take the place of the adjacent element in the **counter-clockwise** direction. An example rotation is shown below:

![](https://assets.leetcode.com/uploads/2021/06/22/explanation_grid.jpg)

Return *the matrix after applying* `k` *cyclic rotations to it*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/19/rod2.png)
<pre>
<strong>Input:</strong> grid = [[40,10],[30,20]], k = 1
<strong>Output:</strong> [[10,20],[40,30]]
<strong>Explanation:</strong> The figures above represent the grid at every state.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid5.png)
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid6.png)
![](https://assets.leetcode.com/uploads/2021/06/10/ringofgrid7.png)
<pre>
<strong>Input:</strong> grid = [[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]], k = 2
<strong>Output:</strong> [[3,4,8,12],[2,11,10,16],[1,7,6,15],[5,9,13,14]]
<strong>Explanation:</strong> The figures above represent the grid at every state.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `2 <= m, n <= 50`
* Both `m` and `n` are **even** integers.
* `1 <= grid[i][j] <= 5000`
* <code>1 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
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
