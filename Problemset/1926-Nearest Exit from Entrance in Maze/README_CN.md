# 1926. 迷宫中离入口最近的出口
给你一个 `m x n` 的迷宫矩阵 `maze` （**下标从 0 开始**），矩阵中有空格子（用 `'.'` 表示）和墙（用 `'+'` 表示）。同时给你迷宫的入口 `entrance` ，用 <code>entrance = [entrance<sub>row</sub>, entrance<sub>col</sub>]</code> 表示你一开始所在格子的行和列。

每一步操作，你可以往 **上**，**下**，**左** 或者 **右** 移动一个格子。你不能进入墙所在的格子，你也不能离开迷宫。你的目标是找到离 `entrance` **最近** 的出口。**出口** 的含义是 `maze` **边界** 上的 **空格子**。`entrance` 格子 **不算** 出口。

请你返回从 `entrance` 到最近出口的最短路径的 **步数** ，如果不存在这样的路径，请你返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/04/nearest1-grid.jpg)
<pre>
<strong>输入:</strong> maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 总共有 3 个出口，分别位于 (1,0)，(0,2) 和 (2,3) 。
一开始，你在入口格子 (1,2) 处。
- 你可以往左移动 2 步到达 (1,0) 。
- 你可以往上移动 1 步到达 (0,2) 。
从入口处没法到达 (2,3) 。
所以，最近的出口是 (0,2) ，距离为 1 步。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/04/nearesr2-grid.jpg)
<pre>
<strong>输入:</strong> maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
<strong>输出:</strong> 2
<strong>解释:</strong> 迷宫中只有 1 个出口，在 (1,2) 处。
(1,0) 不算出口，因为它是入口格子。
初始时，你在入口与格子 (1,0) 处。
- 你可以往右移动 2 步到达 (1,2) 处。
所以，最近的出口为 (1,2) ，距离为 2 步。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/06/04/nearest3-grid.jpg)
<pre>
<strong>输入:</strong> maze = [[".","+"]], entrance = [0,0]
<strong>输出:</strong> -1
<strong>解释:</strong> 这个迷宫中没有出口。
</pre>

#### 提示:
* `maze.length == m`
* `maze[i].length == n`
* `1 <= m, n <= 100`
* `maze[i][j]` 要么是 `'.'` ，要么是 `'+'` 。
* `entrance.length == 2`
* <code>0 <= entrance<sub>row</sub> < m</code>
* <code>0 <= entrance<sub>col</sub> < n</code>
* `entrance` 一定是空格子。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len() as i32;
        let n = maze[0].len() as i32;
        let mut maze = maze;
        let mut cells = VecDeque::from([(entrance[0], entrance[1], 0)]);
        maze[cells[0].0 as usize][cells[0].1 as usize] = '+';

        while let Some((row, col, steps)) = cells.pop_front() {
            if (row != entrance[0] || col != entrance[1])
                && (row == 0 || col == 0 || row == m - 1 || col == n - 1)
            {
                return steps;
            }

            for (r, c) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let row = row + r;
                let col = col + c;

                if row >= 0
                    && row < m
                    && col >= 0
                    && col < n
                    && maze[row as usize][col as usize] == '.'
                {
                    cells.push_back((row, col, steps + 1));
                    maze[row as usize][col as usize] = '+';
                }
            }
        }

        -1
    }
}
```
