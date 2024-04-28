# 778. 水位上升的泳池中游泳
在一个 `n x n` 的整数矩阵 `grid` 中，每一个方格的值 `grid[i][j]` 表示位置 `(i, j)` 的平台高度。

当开始下雨时，在时间为 `t` 时，水池中的水位为 `t` 。你可以从一个平台游向四周相邻的任意一个平台，但是前提是此时水位必须同时淹没这两个平台。假定你可以瞬间移动无限距离，也就是默认在方格内部游动是不耗时的。当然，在你游泳的时候你必须待在坐标方格里面。

你从坐标方格的左上平台 `(0，0)` 出发。返回 *你到达坐标方格的右下平台 `(n-1, n-1)` 所需的最少时间* 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/29/swim1-grid.jpg)
<pre>
<strong>输入:</strong> grid = [[0,2],[1,3]]
<strong>输出:</strong> 3
<strong>解释:</strong>
时间为0时，你位于坐标方格的位置为 (0, 0)。
此时你不能游向任意方向，因为四个相邻方向平台的高度都大于当前时间为 0 时的水位。
等时间到达 3 时，你才可以游向平台 (1, 1). 因为此时的水位是 3，坐标方格中的平台没有比水位 3 更高的，所以你可以游向坐标方格中的任意位置
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/29/swim2-grid-1.jpg)
<pre>
<strong>输入:</strong> grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
<strong>输出:</strong> 16
<strong>解释:</strong> 最终的路线用加粗进行了标记。
我们必须等到时间为 16，此时才能保证平台 (0, 0) 和 (4, 4) 是连通的
</pre>

#### 提示:
* `n == grid.length`
* `n == grid[i].length`
* `1 <= n <= 50`
* <code>0 <= grid[i][j] < n<sup>2</sup></code>
* `grid[i][j]` 中每个值 **均无重复**

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut heap = BinaryHeap::from([(-grid[0][0], 0, 0)]);
        let mut visited = HashSet::new();

        while let Some((t, i, j)) = heap.pop() {
            if i == n - 1 && j == n - 1 {
                return -t;
            }

            if visited.contains(&(i, j)) {
                continue;
            }

            visited.insert((i, j));

            if i > 0 && !visited.contains(&(i - 1, j)) {
                heap.push((t.min(-grid[i - 1][j]), i - 1, j));
            }
            if i < n - 1 && !visited.contains(&(i + 1, j)) {
                heap.push((t.min(-grid[i + 1][j]), i + 1, j));
            }
            if j > 0 && !visited.contains(&(i, j - 1)) {
                heap.push((t.min(-grid[i][j - 1]), i, j - 1));
            }
            if j < n - 1 && !visited.contains(&(i, j + 1)) {
                heap.push((t.min(-grid[i][j + 1]), i, j + 1));
            }
        }

        unreachable!()
    }
}
```
