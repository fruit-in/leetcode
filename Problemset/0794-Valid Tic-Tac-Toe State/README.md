# 794. Valid Tic-Tac-Toe State
Given a Tic-Tac-Toe board as a string array `board`, return `true` if and only if it is possible to reach this board position during the course of a valid tic-tac-toe game.

The board is a `3 x 3` array that consists of characters `' '`, `'X'`, and `'O'`. The `' '` character represents an empty square.

Here are the rules of Tic-Tac-Toe:
* Players take turns placing characters into empty squares `' '`.
* The first player always places `'X'` characters, while the second player always places `'O'` characters.
* `'X'` and `'O'` characters are always placed into empty squares, never filled ones.
* The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
* The game also ends if all squares are non-empty.
* No more moves can be played if the game is over.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe1-grid.jpg)
<pre>
<strong>Input:</strong> board = ["O  ","   ","   "]
<strong>Output:</strong> false
<strong>Explanation:</strong> The first player always plays "X".
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe2-grid.jpg)
<pre>
<strong>Input:</strong> board = ["XOX"," X ","   "]
<strong>Output:</strong> false
<strong>Explanation:</strong> Players take turns making moves.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe3-grid.jpg)
<pre>
<strong>Input:</strong> board = ["XXX","   ","OOO"]
<strong>Output:</strong> false
</pre>

#### Example 4:
![](https://assets.leetcode.com/uploads/2021/05/15/tictactoe4-grid.jpg)
<pre>
<strong>Input:</strong> board = ["XOX","O O","XOX"]
<strong>Output:</strong> true
</pre>

#### Constraints:
* `board.length == 3`
* `board[i].length == 3`
* `board[i][j]` is either `'X'`, `'O'`, or `' '`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board.concat().into_bytes();
        let x_count = board.iter().filter(|&&c| c == b'X').count();
        let o_count = board.iter().filter(|&&c| c == b'O').count();

        (x_count == o_count + 1 && !Self::is_win(b'O', &board))
            || (x_count == o_count && !Self::is_win(b'X', &board))
    }

    fn is_win(player: u8, board: &[u8]) -> bool {
        [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ]
        .iter()
        .any(|&(x, y, z)| [board[x], board[y], board[z]] == [player, player, player])
    }
}
```
