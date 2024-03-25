# 1463. 摘樱桃 II
给你一个 `rows x cols` 的矩阵 `grid` 来表示一块樱桃地。 `grid` 中每个格子的数字表示你能获得的樱桃数目。

你有两个机器人帮你收集樱桃，机器人 1 从左上角格子 `(0,0)` 出发，机器人 2 从右上角格子 `(0, cols-1)` 出发。

请你按照如下规则，返回两个机器人能收集的最多樱桃数目：

* 从格子 `(i,j)` 出发，机器人可以移动到格子 `(i+1, j-1)`，`(i+1, j)` 或者 `(i+1, j+1)` 。
* 当一个机器人经过某个格子时，它会把该格子内所有的樱桃都摘走，然后这个位置会变成空格子，即没有樱桃的格子。
* 当两个机器人同时到达同一个格子时，它们中只有一个可以摘到樱桃。
* 两个机器人在任意时刻都不能移动到 `grid` 外面。
* 两个机器人最后都要到达 `grid` 最底下一行。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_1_1802.png)
<pre>
<strong>输入:</strong> grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
<strong>输出:</strong> 24
<strong>解释:</strong> 机器人 1 和机器人 2 的路径在上图中分别用绿色和蓝色表示。
机器人 1 摘的樱桃数目为 (3 + 2 + 5 + 2) = 12 。
机器人 2 摘的樱桃数目为 (1 + 5 + 5 + 1) = 12 。
樱桃总数为： 12 + 12 = 24 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/04/23/sample_2_1802.png)
<pre>
<strong>输入:</strong> grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]
<strong>输出:</strong> 28
<strong>解释:</strong> 机器人 1 和机器人 2 的路径在上图中分别用绿色和蓝色表示。
机器人 1 摘的樱桃数目为 (1 + 9 + 5 + 2) = 17 。
机器人 2 摘的樱桃数目为 (1 + 3 + 4 + 3) = 11 。
樱桃总数为： 17 + 11 = 28 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,0,0,3],[0,0,0,3],[0,0,3,3],[9,0,3,3]]
<strong>输出:</strong> 22
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> grid = [[1,1],[1,1]]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `rows == grid.length`
* `cols == grid[i].length`
* `2 <= rows, cols <= 70`
* `0 <= grid[i][j] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut curr_row = vec![vec![None; cols]; cols];
        curr_row[0][cols - 1] = Some(grid[0][0] + grid[0][cols - 1]);

        for i in 0..rows - 1 {
            let mut next_row = vec![vec![None; cols]; cols];

            for j0 in 0..cols {
                for j1 in 0..cols {
                    if let Some(x) = curr_row[j0][j1] {
                        for c0 in j0.saturating_sub(1)..cols.min(j0 + 2) {
                            for c1 in j1.saturating_sub(1)..cols.min(j1 + 2) {
                                next_row[c0][c1] = Some(next_row[c0][c1].unwrap_or(0).max(
                                    x + grid[i + 1][c0] + grid[i + 1][c1] * (c0 != c1) as i32,
                                ));
                            }
                        }
                    }
                }
            }

            curr_row = next_row;
        }

        curr_row
            .iter()
            .flatten()
            .map(|x| x.unwrap_or(0))
            .max()
            .unwrap()
    }
}
```
