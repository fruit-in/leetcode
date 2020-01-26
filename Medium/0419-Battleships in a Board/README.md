# 419. Battleships in a Board
Given an 2D board, count how many battleships are in it. The battleships are represented with ```'X'```s, empty slots are represented with ```'.'```s. You may assume the following rules:
* You receive a valid board, made of only battleships or empty slots.
* Battleships can only be placed horizontally or vertically. In other words, they can only be made of the shape ```1xN``` (1 row, N columns) or ```Nx1``` (N rows, 1 column), where N can be of any size.
* At least one horizontal or vertical cell separates between two battleships - there are no adjacent battleships.

#### Example:
```
X..X
...X
...X
```
In the above board there are 2 battleships.

#### Invalid Example:
```
...X
XXXX
...X
```
This is an invalid board that you will not receive - as battleships will always have a cell separating between them.

#### Follow up:
Could you do it in **one-pass**, using only **O(1) extra memory** and **without modifying** the value of the board?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ret = 0;

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' &&
                (i + 1 == board.len() || board[i + 1][j] == '.') &&
                (j + 1 == board[0].len() || board[i][j + 1] != 'X') {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```
