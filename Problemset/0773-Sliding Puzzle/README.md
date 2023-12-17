# 773. Sliding Puzzle
On an `2 x 3` board, there are five tiles labeled from `1` to `5`, and an empty square represented by `0`. A **move** consists of choosing `0` and a 4-directionally adjacent number and swapping it.

The state of the board is solved if and only if the board is `[[1,2,3],[4,5,0]]`.

Given the puzzle board `board`, return *the least number of moves required so that the state of the board is solved*. If it is impossible for the state of the board to be solved, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/29/slide1-grid.jpg)
<pre>
<strong>Input:</strong> board = [[1,2,3],[4,0,5]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Swap the 0 and the 5 in one move.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/29/slide2-grid.jpg)
<pre>
<strong>Input:</strong> board = [[1,2,3],[5,4,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> No number of moves will make the board solved.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/06/29/slide3-grid.jpg)
<pre>
<strong>Input:</strong> board = [[4,1,2],[5,0,3]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> 5 is the smallest number of moves that solves the board.
An example path:
After move 0: [[4,1,2],[5,0,3]]
After move 1: [[4,1,2],[0,5,3]]
After move 2: [[0,1,2],[4,5,3]]
After move 3: [[1,0,2],[4,5,3]]
After move 4: [[1,2,0],[4,5,3]]
After move 5: [[1,2,3],[4,5,0]]
</pre>

#### Constraints:
* `board.length == 2`
* `board[i].length == 3`
* `0 <= board[i][j] <= 5`
* Each value `board[i][j]` is **unique**.

## Solutions (Rust)

### 1. Solution
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
