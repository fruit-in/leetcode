# 773. 滑动谜题
在一个 `2 x 3` 的板上（`board`）有 5 块砖瓦，用数字 `1~5` 来表示, 以及一块空缺用 `0` 来表示。一次 **移动** 定义为选择 `0` 与一个相邻的数字（上下左右）进行交换.

最终当板 `board` 的结果是 `[[1,2,3],[4,5,0]]` 谜板被解开。

给出一个谜板的初始状态 `board` ，返回最少可以通过多少次移动解开谜板，如果不能解开谜板，则返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/29/slide1-grid.jpg)
<pre>
<strong>输入:</strong> board = [[1,2,3],[4,0,5]]
<strong>输出:</strong> 1
<strong>解释:</strong> 交换 0 和 5 ，1 步完成
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/29/slide2-grid.jpg)
<pre>
<strong>输入:</strong> board = [[1,2,3],[5,4,0]]
<strong>输出:</strong> -1
<strong>解释:</strong> 没有办法完成谜板
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/06/29/slide3-grid.jpg)
<pre>
<strong>输入:</strong> board = [[4,1,2],[5,0,3]]
<strong>输出:</strong> 5
<strong>解释:</strong>
最少完成谜板的最少移动次数是 5 ，
一种移动路径:
尚未移动: [[4,1,2],[5,0,3]]
移动 1 次: [[4,1,2],[0,5,3]]
移动 2 次: [[0,1,2],[4,5,3]]
移动 3 次: [[1,0,2],[4,5,3]]
移动 4 次: [[1,2,0],[4,5,3]]
移动 5 次: [[1,2,3],[4,5,0]]
</pre>

#### 提示:
* `board.length == 2`
* `board[i].length == 3`
* `0 <= board[i][j] <= 5`
* `board[i][j]` 中每个值都 **不同**

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::from([[
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
        ]]);
        let mut moves_required = HashMap::from([(queue[0], 0)]);

        while let Some(board) = queue.pop_front() {
            if board == [1, 2, 3, 4, 5, 0] {
                return moves_required[&board];
            }

            let mut boards = vec![];
            let mut tmp = board;

            if board[0] == 0 {
                tmp.swap(0, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(0, 3);
                boards.push(tmp);
            } else if board[1] == 0 {
                tmp.swap(1, 0);
                boards.push(tmp);
                tmp = board;
                tmp.swap(1, 2);
                boards.push(tmp);
                tmp = board;
                tmp.swap(1, 4);
                boards.push(tmp);
            } else if board[2] == 0 {
                tmp.swap(2, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(2, 5);
                boards.push(tmp);
            } else if board[3] == 0 {
                tmp.swap(3, 0);
                boards.push(tmp);
                tmp = board;
                tmp.swap(3, 4);
                boards.push(tmp);
            } else if board[4] == 0 {
                tmp.swap(4, 1);
                boards.push(tmp);
                tmp = board;
                tmp.swap(4, 3);
                boards.push(tmp);
                tmp = board;
                tmp.swap(4, 5);
                boards.push(tmp);
            } else {
                tmp.swap(5, 2);
                boards.push(tmp);
                tmp = board;
                tmp.swap(5, 4);
                boards.push(tmp);
            }

            for i in 0..boards.len() {
                if !moves_required.contains_key(&boards[i]) {
                    queue.push_back(boards[i]);
                    moves_required.insert(boards[i], moves_required[&board] + 1);
                }
            }
        }

        -1
    }
}
```
