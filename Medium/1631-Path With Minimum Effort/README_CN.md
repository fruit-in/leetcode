# 1631. 最小体力消耗路径
你准备参加一场远足活动。给你一个二维 `rows x columns` 的地图 `heights` ，其中 `heights[row][col]` 表示格子 `(row, col)` 的高度。一开始你在最左上角的格子 `(0, 0)` ，且你希望去最右下角的格子 `(rows-1, columns-1)` （注意下标从 **0** 开始编号）。你每次可以往 **上**，**下**，**左**，**右** 四个方向之一移动，你想要找到耗费 **体力** 最小的一条路径。

一条路径耗费的 **体力值** 是路径上相邻格子之间 **高度差绝对值** 的 **最大值** 决定的。

请你返回从左上角走到右下角的最小 **体力消耗值** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/04/ex1.png)
<pre>
<strong>输入:</strong> heights = [[1,2,2],[3,8,2],[5,3,5]]
<strong>输出:</strong> 2
<strong>解释:</strong> 路径 [1,3,5,3,5] 连续格子的差值绝对值最大为 2 。
这条路径比路径 [1,2,2,2,5] 更优，因为另一条路径差值最大值为 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/04/ex2.png)
<pre>
<strong>输入:</strong> heights = [[1,2,3],[3,8,4],[5,3,5]]
<strong>输出:</strong> 1
<strong>解释:</strong> 路径 [1,2,3,4,5] 的相邻格子差值绝对值最大为 1 ，比路径 [1,3,5,3,5] 更优。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/10/04/ex3.png)
<pre>
<strong>输入:</strong> heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
<strong>输出:</strong> 0
<strong>解释:</strong> 上图所示路径不需要消耗任何体力。
</pre>

#### 提示:
* `rows == heights.length`
* `columns == heights[i].length`
* `1 <= rows, columns <= 100`
* <code>1 <= heights[i][j] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut min_efforts = vec![vec![i32::MAX; heights[0].len()]; heights.len()];
        let mut cells = vec![(0, 0)];
        min_efforts[0][0] = 0;

        while let Some((i, j)) = cells.pop() {
            if i > 0 {
                let effort = (heights[i][j] - heights[i - 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i - 1][j] {
                    min_efforts[i - 1][j] = effort;
                    cells.push((i - 1, j));
                }
            }
            if i < heights.len() - 1 {
                let effort = (heights[i][j] - heights[i + 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i + 1][j] {
                    min_efforts[i + 1][j] = effort;
                    cells.push((i + 1, j));
                }
            }
            if j > 0 {
                let effort = (heights[i][j] - heights[i][j - 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j - 1] {
                    min_efforts[i][j - 1] = effort;
                    cells.push((i, j - 1));
                }
            }
            if j < heights[0].len() - 1 {
                let effort = (heights[i][j] - heights[i][j + 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j + 1] {
                    min_efforts[i][j + 1] = effort;
                    cells.push((i, j + 1));
                }
            }
        }

        *min_efforts.last().unwrap().last().unwrap()
    }
}
```
