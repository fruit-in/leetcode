# 999. Available Captures for Rook
On an 8 x 8 chessboard, there is one white rook.  There also may be empty squares, white bishops, and black pawns.  These are given as characters 'R', '.', 'B', and 'p' respectively. Uppercase characters represent white pieces, and lowercase characters represent black pieces.

The rook moves as in the rules of Chess: it chooses one of four cardinal directions (north, east, west, and south), then moves in that direction until it chooses to stop, reaches the edge of the board, or captures an opposite colored pawn by moving to the same square it occupies.  Also, rooks cannot move into the same square as other friendly bishops.

Return the number of pawns the rook can capture in one move.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/20/1253_example_1_improved.PNG)
<pre>
<strong>Input:</strong> [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","R",".",".",".","p"],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
In this example the rook is able to capture all the pawns.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/02/19/1253_example_2_improved.PNG)
<pre>
<strong>Input:</strong> [[".",".",".",".",".",".",".","."],[".","p","p","p","p","p",".","."],[".","p","p","B","p","p",".","."],[".","p","B","R","B","p",".","."],[".","p","p","B","p","p",".","."],[".","p","p","p","p","p",".","."],[".",".",".",".",".",".",".","."],[".",".",".",".",".",".",".","."]]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Bishops are blocking the rook to capture any pawn.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/02/20/1253_example_3_improved.PNG)
<pre>
<strong>Input:</strong> [[".",".",".",".",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".","p",".",".",".","."],["p","p",".","R",".","p","B","."],[".",".",".",".",".",".",".","."],[".",".",".","B",".",".",".","."],[".",".",".","p",".",".",".","."],[".",".",".",".",".",".",".","."]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The rook can capture the pawns at positions b5, d6 and f5.
</pre>

#### Note:
1. ```board.length == board[i].length == 8```
2. ```board[i][j]``` is either ```'R'```, ```'.'```, ```'B'```, or ```'p'```
3. There is exactly one cell with ```board[i][j] == 'R'```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut rook = (0, 0);
        for i in 0..8 {
            if let Some(j) = board[i].iter().position(|&c| c == 'R') {
                rook = (i, j);
                break;
            }
        }

        let mut flag = [0, 0, 0, 0];
        for i in 0..rook.0 {
            match board[i][rook.1] {
                'p' => flag[0] = 1,
                'B' => flag[0] = 0,
                _ => (),
            }
        }
        for i in ((rook.0 + 1)..8).rev() {
            match board[i][rook.1] {
                'p' => flag[1] = 1,
                'B' => flag[1] = 0,
                _ => (),
            }
        }
        for i in 0..rook.1 {
            match board[rook.0][i] {
                'p' => flag[2] = 1,
                'B' => flag[2] = 0,
                _ => (),
            }
        }
        for i in ((rook.1 + 1)..8).rev() {
            match board[rook.0][i] {
                'p' => flag[3] = 1,
                'B' => flag[3] = 0,
                _ => (),
            }
        }

        flag.iter().sum()
    }
}
```
