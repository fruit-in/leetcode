# 1263. 推箱子
「推箱子」是一款风靡全球的益智小游戏，玩家需要将箱子推到仓库中的目标位置。

游戏地图用大小为 `m x n` 的网格 `grid` 表示，其中每个元素可以是墙、地板或者是箱子。

现在你将作为玩家参与游戏，按规则将箱子 `'B'` 移动到目标位置 `'T'` ：
* 玩家用字符 `'S'` 表示，只要他在地板上，就可以在网格中向上、下、左、右四个方向移动。
* 地板用字符 `'.'` 表示，意味着可以自由行走。
* 墙用字符 `'#'` 表示，意味着障碍物，不能通行。
* 箱子仅有一个，用字符 `'B'` 表示。相应地，网格上有一个目标位置 `'T'`。
* 玩家需要站在箱子旁边，然后沿着箱子的方向进行移动，此时箱子会被移动到相邻的地板单元格。记作一次「推动」。
* 玩家无法越过箱子。

返回将箱子推到目标位置的最小 **推动** 次数，如果无法做到，请返回 `-1`。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/11/06/sample_1_1620.png)
<pre>
<strong>输入:</strong> grid = [["#","#","#","#","#","#"],
               ["#","T","#","#","#","#"],
               ["#",".",".","B",".","#"],
               ["#",".","#","#",".","#"],
               ["#",".",".",".","S","#"],
               ["#","#","#","#","#","#"]]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们只需要返回推箱子的次数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [["#","#","#","#","#","#"],
               ["#","T","#","#","#","#"],
               ["#",".",".","B",".","#"],
               ["#","#","#","#",".","#"],
               ["#",".",".",".","S","#"],
               ["#","#","#","#","#","#"]]
<strong>输出:</strong> -1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [["#","#","#","#","#","#"],
               ["#","T",".",".","#","#"],
               ["#",".","#","B",".","#"],
               ["#",".",".",".",".","#"],
               ["#",".",".",".","S","#"],
               ["#","#","#","#","#","#"]]
<strong>输出:</strong> 5
<strong>解释:</strong> 向下、向左、向左、向上再向上。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 20`
* `grid` 仅包含字符 `'.'`, `'#'`,  `'S'` , `'T'`, 以及 `'B'`。
* `grid` 中 `'S'`, `'B'` 和 `'T'` 各只能出现一个。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited_player = HashSet::new();
        let mut stack_player = vec![];
        let mut visited_box = HashSet::new();
        let mut deque_box = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 'S' {
                    visited_player.insert((i, j));
                    stack_player.push((i, j));
                    break;
                }
            }
        }

        while let Some((i, j)) = stack_player.pop() {
            if i > 0 && grid[i - 1][j] == 'B' && !visited_box.contains(&(i - 1, j, 'U')) {
                visited_box.insert((i - 1, j, 'U'));
                deque_box.push_back((i - 1, j, 'U', 0));
            }
            if i < m - 1 && grid[i + 1][j] == 'B' && !visited_box.contains(&(i + 1, j, 'D')) {
                visited_box.insert((i + 1, j, 'D'));
                deque_box.push_back((i + 1, j, 'D', 0));
            }
            if j > 0 && grid[i][j - 1] == 'B' && !visited_box.contains(&(i, j - 1, 'L')) {
                visited_box.insert((i, j - 1, 'L'));
                deque_box.push_back((i, j - 1, 'L', 0));
            }
            if j < n - 1 && grid[i][j + 1] == 'B' && !visited_box.contains(&(i, j + 1, 'R')) {
                visited_box.insert((i, j + 1, 'R'));
                deque_box.push_back((i, j + 1, 'R', 0));
            }

            if i > 0
                && grid[i - 1][j] != '#'
                && grid[i - 1][j] != 'B'
                && !visited_player.contains(&(i - 1, j))
            {
                visited_player.insert((i - 1, j));
                stack_player.push((i - 1, j));
            }
            if i < m - 1
                && grid[i + 1][j] != '#'
                && grid[i + 1][j] != 'B'
                && !visited_player.contains(&(i + 1, j))
            {
                visited_player.insert((i + 1, j));
                stack_player.push((i + 1, j));
            }
            if j > 0
                && grid[i][j - 1] != '#'
                && grid[i][j - 1] != 'B'
                && !visited_player.contains(&(i, j - 1))
            {
                visited_player.insert((i, j - 1));
                stack_player.push((i, j - 1));
            }
            if j < n - 1
                && grid[i][j + 1] != '#'
                && grid[i][j + 1] != 'B'
                && !visited_player.contains(&(i, j + 1))
            {
                visited_player.insert((i, j + 1));
                stack_player.push((i, j + 1));
            }
        }

        while let Some((i, j, s, pushes)) = deque_box.pop_front() {
            if grid[i][j] == 'T' {
                return pushes;
            }

            let mut bi = i;
            let mut bj = j;

            if s == 'U' && i > 0 && grid[i - 1][j] != '#' {
                bi -= 1;
            } else if s == 'D' && i < m - 1 && grid[i + 1][j] != '#' {
                bi += 1;
            } else if s == 'L' && j > 0 && grid[i][j - 1] != '#' {
                bj -= 1;
            } else if s == 'R' && j < n - 1 && grid[i][j + 1] != '#' {
                bj += 1;
            } else {
                continue;
            }

            visited_player = HashSet::from([(i, j)]);
            stack_player = vec![(i, j)];

            while let Some((i, j)) = stack_player.pop() {
                if i > 0 && i - 1 == bi && j == bj && !visited_box.contains(&(bi, bj, 'U')) {
                    visited_box.insert((bi, bj, 'U'));
                    deque_box.push_back((bi, bj, 'U', pushes + 1));
                }
                if i < m - 1 && i + 1 == bi && j == bj && !visited_box.contains(&(bi, bj, 'D')) {
                    visited_box.insert((bi, bj, 'D'));
                    deque_box.push_back((bi, bj, 'D', pushes + 1));
                }
                if j > 0 && i == bi && j - 1 == bj && !visited_box.contains(&(bi, bj, 'L')) {
                    visited_box.insert((bi, bj, 'L'));
                    deque_box.push_back((bi, bj, 'L', pushes + 1));
                }
                if j < n - 1 && i == bi && j + 1 == bj && !visited_box.contains(&(bi, bj, 'R')) {
                    visited_box.insert((bi, bj, 'R'));
                    deque_box.push_back((bi, bj, 'R', pushes + 1));
                }

                if i > 0
                    && grid[i - 1][j] != '#'
                    && (i - 1 != bi || j != bj)
                    && !visited_player.contains(&(i - 1, j))
                {
                    visited_player.insert((i - 1, j));
                    stack_player.push((i - 1, j));
                }
                if i < m - 1
                    && grid[i + 1][j] != '#'
                    && (i + 1 != bi || j != bj)
                    && !visited_player.contains(&(i + 1, j))
                {
                    visited_player.insert((i + 1, j));
                    stack_player.push((i + 1, j));
                }
                if j > 0
                    && grid[i][j - 1] != '#'
                    && (i != bi || j - 1 != bj)
                    && !visited_player.contains(&(i, j - 1))
                {
                    visited_player.insert((i, j - 1));
                    stack_player.push((i, j - 1));
                }
                if j < n - 1
                    && grid[i][j + 1] != '#'
                    && (i != bi || j + 1 != bj)
                    && !visited_player.contains(&(i, j + 1))
                {
                    visited_player.insert((i, j + 1));
                    stack_player.push((i, j + 1));
                }
            }
        }

        -1
    }
}
```
