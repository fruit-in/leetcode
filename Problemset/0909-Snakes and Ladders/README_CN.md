# 909. 蛇梯棋
给你一个大小为 `n x n` 的整数矩阵 `board` ，方格按从 `1` 到 <code>n<sup>2</sup></code> 编号，编号遵循 [转行交替方式](https://baike.baidu.com/item/%E7%89%9B%E8%80%95%E5%BC%8F%E8%BD%AC%E8%A1%8C%E4%B9%A6%E5%86%99%E6%B3%95/17195786) ，**从左下角开始** （即，从 `board[n - 1][0]` 开始）每一行交替方向。

玩家从棋盘上的方格 `1` （总是在最后一行、第一列）开始出发。

每一回合，玩家需要从当前方格 `curr` 开始出发，按下述要求前进：

* 选定目标方格 `next` ，目标方格的编号符合范围 <code>[curr + 1, min(curr + 6, n<sup>2</sup>)]</code> 。
    * 该选择模拟了掷 **六面体骰子** 的情景，无论棋盘大小如何，玩家最多只能有 6 个目的地。
* 传送玩家：如果目标方格 `next` 处存在蛇或梯子，那么玩家会传送到蛇或梯子的目的地。否则，玩家传送到目标方格 `next` 。
* 当玩家到达编号 <code>n<sup>2</sup></code> 的方格时，游戏结束。

`r` 行 `c` 列的棋盘，按前述方法编号，棋盘格中可能存在 “蛇” 或 “梯子”；如果 `board[r][c] != -1`，那个蛇或梯子的目的地将会是 `board[r][c]`。编号为 `1` 和 <code>n<sup>2</sup></code> 的方格上没有蛇或梯子。

注意，玩家在每回合的前进过程中最多只能爬过蛇或梯子一次：就算目的地是另一条蛇或梯子的起点，玩家也 **不能** 继续移动。

* 举个例子，假设棋盘是 `[[-1,4],[-1,3]]` ，第一次移动，玩家的目标方格是 `2` 。那么这个玩家将会顺着梯子到达方格 `3` ，但 **不能** 顺着方格 3 上的梯子前往方格 `4` 。

返回达到编号为 <code>n<sup>2</sup></code> 的方格所需的最少移动次数，如果不可能，则返回 `-1`。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/09/23/snakes.png)
<pre>
<strong>输入:</strong> board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
<strong>输出:</strong> 4
<strong>解释:</strong>
首先，从方格 1 [第 5 行，第 0 列] 开始。
先决定移动到方格 2 ，并必须爬过梯子移动到到方格 15 。
然后决定移动到方格 17 [第 3 行，第 4 列]，必须爬过蛇到方格 13 。
接着决定移动到方格 14 ，且必须通过梯子移动到方格 35 。
最后决定移动到方格 36 , 游戏结束。
可以证明需要至少 4 次移动才能到达最后一个方格，所以答案是 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> board = [[-1,-1],[-1,3]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `n == board.length == board[i].length`
* `2 <= n <= 20`
* `grid[i][j]` 的值是 `-1` 或在范围 <code>[1, n<sup>2</sup>]</code> 内
* 编号为 `1` 和 <code>n<sup>2</sup></code> 的方格上没有蛇或梯子

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut dests = vec![None; n * n];
        let mut visited = HashSet::from([0]);
        let mut deque = VecDeque::from([(0, 0)]);

        for i in 1..dests.len() - 1 {
            let r = n - 1 - i / n;
            let c = match i / n % 2 {
                0 => i % n,
                _ => n - 1 - i % n,
            };

            if board[r][c] > 0 {
                dests[i] = Some(board[r][c] as usize - 1);
            }
        }

        while let Some((i, m)) = deque.pop_front() {
            if i == n * n - 1 {
                return m;
            }

            for j in i + 1..(i + 7).min(n * n) {
                let j = dests[j].unwrap_or(j);

                if !visited.contains(&j) {
                    visited.insert(j);
                    deque.push_back((j, m + 1));
                }
            }
        }

        -1
    }
}
```
