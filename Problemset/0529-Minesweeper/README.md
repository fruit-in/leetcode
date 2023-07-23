# 529. Minesweeper
Let's play the minesweeper game ([Wikipedia](https://en.wikipedia.org/wiki/Minesweeper_(video_game)), [online game](http://minesweeperonline.com/#))!

You are given a 2D char matrix representing the game board. **'M'** represents an **unrevealed** mine, **'E'** represents an **unrevealed** empty square, **'B'** represents a **revealed** blank square that has no adjacent (above, below, left, right, and all 4 diagonals) mines, **digit** ('1' to '8') represents how many mines are adjacent to this **revealed** square, and finally **'X'** represents a **revealed** mine.

Now given the next click position (row and column indices) among all the **unrevealed** squares ('M' or 'E'), return the board after revealing this position according to the following rules:
1. If a mine ('M') is revealed, then the game is over - change it to **'X'**.
2. If an empty square ('E') with **no adjacent mines** is revealed, then change it to revealed blank ('B') and all of its adjacent **unrevealed** squares should be revealed recursively.
3. If an empty square ('E') with **at least one adjacent mine** is revealed, then change it to a digit ('1' to '8') representing the number of adjacent mines.
4. Return the board when no more squares will be revealed.

#### Example 1:
<pre>
<strong>Input:</strong>
[['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'M', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E']]

Click : [3,0]
<strong>Output:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]
<strong>Explanation:</strong>
<img src='https://assets.leetcode.com/uploads/2018/10/12/minesweeper_example_1.png'>
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]

Click : [1,2]
<strong>Output:</strong>
[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'X', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]
<strong>Explanation:</strong>
<img src='https://assets.leetcode.com/uploads/2018/10/12/minesweeper_example_2.png'>
</pre>

#### Note:
1. The range of the input matrix's height and width is [1,50].
2. The click position will only be an unrevealed square ('M' or 'E'), which also means the input board contains at least one clickable square.
3. The input board won't be a stage when game is over (some mines have been revealed).
4. For simplicity, not mentioned rules should be ignored in this problem. For example, you **don't** need to reveal all the unrevealed mines when the game is over, consider any cases that you will win the game or flag any squares.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let mut clicks = vec![click];
        let h = board.len() as i32;
        let w = board[0].len() as i32;

        while let Some(click) = clicks.pop() {
            let r = click[0];
            let c = click[1];

            match board[r as usize][c as usize] {
                'M' => board[r as usize][c as usize] = 'X',
                'E' => {
                    let mut mines = 0;
                    let mut empties = Vec::new();

                    for i in (r - 1).max(0)..(r + 2).min(h) {
                        for j in (c - 1).max(0)..(c + 2).min(w) {
                            if (i != r || j != c) {
                                match board[i as usize][j as usize] {
                                    'M' => mines += 1,
                                    'E' => empties.push(vec![i, j]),
                                    _ => (),
                                }
                            }
                        }
                    }

                    board[r as usize][c as usize] = match mines {
                        0 => {
                            clicks.append(&mut empties);
                            'B'
                        },
                        m => (m as u8 + b'0') as char,
                    }
                },
                _ => (),
            }
        }

        board
    }
}
```
