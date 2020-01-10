# 1275. Find Winner on a Tic Tac Toe Game
Tic-tac-toe is played by two players *A* and *B* on a *3* x *3* grid.

Here are the rules of Tic-Tac-Toe:
* Players take turns placing characters into empty squares (" ").
* The first player *A* always places "X" characters, while the second player *B* always places "O" characters.
* "X" and "O" characters are always placed into empty squares, never on filled ones.
* The game ends when there are 3 of the same (non-empty) character filling any row, column, or diagonal.
* The game also ends if all squares are non-empty.
* No more moves can be played if the game is over.

Given an array ```moves``` where each element is another array of size 2 corresponding to the row and column of the grid where they mark their respective character in the order in which *A* and *B* play.

Return the winner of the game if it exists (*A* or *B*), in case the game ends in a draw return "Draw", if there are still movements to play return "Pending".

You can assume that ```moves``` is **valid** (It follows the rules of Tic-Tac-Toe), the grid is initially empty and *A* will play **first**.

#### Example 1:
<pre>
<strong>Input:</strong> moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
<strong>Output:</strong> "A"
<strong>Explanation:</strong> "A" wins, he always plays first.
"X  "    "X  "    "X  "    "X  "    "<strong>X</strong>  "
"   " -> "   " -> " X " -> " X " -> " <strong>X</strong> "
"   "    "O  "    "O  "    "OO "    "OO<strong>X</strong>"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
<strong>Output:</strong> "B"
<strong>Explanation:</strong> "B" wins.
"X  "    "X  "    "XX "    "XXO"    "XXO"    "XX<strong>O</strong>"
"   " -> " O " -> " O " -> " O " -> "XO " -> "X<strong>O</strong> "
"   "    "   "    "   "    "   "    "   "    "<strong>O</strong>  "
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
<strong>Output:</strong> "Draw"
<strong>Explanation:</strong> The game ends in a draw since there are no moves to make.
"XXO"
"OOX"
"XOX"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> moves = [[0,0],[1,1]]
<strong>Output:</strong> "Pending"
<strong>Explanation:</strong> The game has not finished yet.
"X  "
" O "
"   "
</pre>

#### Constraints:
* ```1 <= moves.length <= 9```
* ```1 <= moves.length <= 9```
* ```0 <= moves[i][j] <= 2```
* There are no repeated elements on ```moves```.
* ```moves``` follow the rules of tic tac toe.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut scores = [0; 8];

        for (i, mv) in moves.iter().enumerate() {
            let player = i as i32 % 2 * 2 - 1;
            scores[mv[0] as usize] += player;
            scores[mv[1] as usize + 3] += player;
            if *mv == vec![0, 0] || *mv == vec![1, 1] || *mv == vec![2, 2] {
                scores[6] += player;
            }
            if *mv == vec![0, 2] || *mv == vec![1, 1] || *mv == vec![2, 0] {
                scores[7] += player;
            }
        }

        if scores.contains(&-3) {
            "A".to_string()
        } else if scores.contains(&3) {
            "B".to_string()
        } else if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}
```
