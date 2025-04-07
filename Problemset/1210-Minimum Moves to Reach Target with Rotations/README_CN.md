# 1210. 穿过迷宫的最少移动次数
你还记得那条风靡全球的贪吃蛇吗？

我们在一个 `n*n` 的网格上构建了新的迷宫地图，蛇的长度为 2，也就是说它会占去两个单元格。蛇会从左上角（`(0, 0)` 和 `(0, 1)`）开始移动。我们用 `0` 表示空单元格，用 1 表示障碍物。蛇需要移动到迷宫的右下角（`(n-1, n-2)` 和 `(n-1, n-1)`）。

每次移动，蛇可以这样走：
* 如果没有障碍，则向右移动一个单元格。并仍然保持身体的水平／竖直状态。
* 如果没有障碍，则向下移动一个单元格。并仍然保持身体的水平／竖直状态。
* 如果它处于水平状态并且其下面的两个单元都是空的，就顺时针旋转 90 度。蛇从（`(r, c)`、`(r, c+1)`）移动到 （`(r, c)`、`(r+1, c)`）。

![](https://assets.leetcode.com/uploads/2019/09/24/image-2.png)

* 如果它处于竖直状态并且其右面的两个单元都是空的，就逆时针旋转 90 度。蛇从（`(r, c)`、`(r+1, c)`）移动到（`(r, c)`、`(r, c+1)`）。

![](https://assets.leetcode.com/uploads/2019/09/24/image-1.png)

返回蛇抵达目的地所需的最少移动次数。

如果无法到达目的地，请返回 `-1`。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/09/24/image.png)
<pre>
<strong>输入:</strong> grid = [[0,0,0,0,0,1],
               [1,1,0,0,1,0],
               [0,0,0,0,1,1],
               [0,0,1,0,1,0],
               [0,1,1,0,0,0],
               [0,1,1,0,0,0]]
<strong>输出:</strong> 11
<strong>解释:</strong>
一种可能的解决方案是 [右, 右, 顺时针旋转, 右, 下, 下, 下, 下, 逆时针旋转, 右, 下]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[0,0,1,1,1,1],
               [0,0,0,0,1,1],
               [1,1,0,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,1],
               [1,1,1,0,0,0]]
<strong>输出:</strong> 9
</pre>

#### 提示:
* `2 <= n <= 100`
* `0 <= grid[i][j] <= 1`
* 蛇保证从空单元格开始出发。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut deque = VecDeque::from([(0, 0, true, 0)]);
        let mut visited = HashSet::from([(0, 0, true)]);

        while let Some((r, c, horizontal, step)) = deque.pop_front() {
            if r == n - 1 && c == n - 2 && horizontal {
                return step;
            }

            if horizontal {
                if c < n - 2 && grid[r][c + 2] == 0 && !visited.contains(&(r, c + 1, true)) {
                    deque.push_back((r, c + 1, true, step + 1));
                    visited.insert((r, c + 1, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r + 1, c, true))
                {
                    deque.push_back((r + 1, c, true, step + 1));
                    visited.insert((r + 1, c, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, false))
                {
                    deque.push_back((r, c, false, step + 1));
                    visited.insert((r, c, false));
                }
            } else {
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c + 1, false))
                {
                    deque.push_back((r, c + 1, false, step + 1));
                    visited.insert((r, c + 1, false));
                }
                if r < n - 2 && grid[r + 2][c] == 0 && !visited.contains(&(r + 1, c, false)) {
                    deque.push_back((r + 1, c, false, step + 1));
                    visited.insert((r + 1, c, false));
                }
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, true))
                {
                    deque.push_back((r, c, true, step + 1));
                    visited.insert((r, c, true));
                }
            }
        }

        -1
    }
}
```
