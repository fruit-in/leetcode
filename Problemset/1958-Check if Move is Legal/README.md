# 1958. Check if Move is Legal
You are given a **0-indexed** `8 x 8` grid `board`, where `board[r][c]` represents the cell `(r, c)` on a game board. On the board, free cells are represented by `'.'`, white cells are represented by `'W'`, and black cells are represented by `'B'`.

Each move in this game consists of choosing a free cell and changing it to the color you are playing as (either white or black). However, a move is only **legal** if, after changing it, the cell becomes the **endpoint of a good line** (horizontal, vertical, or diagonal).

A **good line** is a line of **three or more cells (including the endpoints)** where the endpoints of the line are **one color**, and the remaining cells in the middle are the **opposite color** (no cells in the line are free). You can find examples for good lines in the figure below:

![](https://assets.leetcode.com/uploads/2021/07/22/goodlines5.png)

Given two integers `rMove` and `cMove` and a character `color` representing the color you are playing as (white or black), return `true` *if changing cell* `(rMove, cMove)` *to color* `color` *is a **legal** move, or* `false` *if it is not legal*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/10/grid11.png)
<pre>
<strong>Input:</strong> board = [[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],[".",".",".","W",".",".",".","."],["W","B","B",".","W","W","W","B"],[".",".",".","B",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","W",".",".",".","."]], rMove = 4, cMove = 3, color = "B"
<strong>Output:</strong> true
<strong>Explanation:</strong> '.', 'W', and 'B' are represented by the colors blue, white, and black respectively, and cell (rMove, cMove) is marked with an 'X'.
The two good lines with the chosen cell as an endpoint are annotated above with the red rectangles.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/10/grid2.png)
<pre>
<strong>Input:</strong> board = [[".",".",".",".",".",".",".","."],[".","B",".",".","W",".",".","."],[".",".","W",".",".",".",".","."],[".",".",".","W","B",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".","B","W",".","."],[".",".",".",".",".",".","W","."],[".",".",".",".",".",".",".","B"]], rMove = 4, cMove = 4, color = "W"
<strong>Output:</strong> false
<strong>Explanation:</strong> While there are good lines with the chosen cell as a middle cell, there are no good lines with the chosen cell as an endpoint.
</pre>

#### Constraints:
* `board.length == board[r].length == 8`
* `0 <= rMove, cMove < 8`
* `board[rMove][cMove] == '.'`
* `color` is either `'B'` or `'W'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let r_move = r_move;
        let c_move = c_move;

        for direct in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let mut r = r_move + direct.0;
            let mut c = c_move + direct.1;

            if r < 0
                || r > 7
                || c < 0
                || c > 7
                || board[r as usize][c as usize] == '.'
                || board[r as usize][c as usize] == color
            {
                continue;
            }

            r += direct.0;
            c += direct.1;

            while r >= 0 && r < 8 && c >= 0 && c < 8 && board[r as usize][c as usize] != '.' {
                if board[r as usize][c as usize] == color {
                    return true;
                }

                r += direct.0;
                c += direct.1;
            }
        }

        false
    }
}
```
