# 289. Game of Life
According to the [Wikipedia's article](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life): "The **Game of Life**, also known simply as **Life**, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

Given a *board* with *m* by *n* cells, each cell has an initial state *live* (1) or *dead* (0). Each cell interacts with its [eight neighbors](https://en.wikipedia.org/wiki/Moore_neighborhood) (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
1. Any live cell with fewer than two live neighbors dies, as if caused by under-population.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by over-population..
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

Write a function to compute the next state (after one update) of the board given its current state. The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously.

#### Example:
<pre>
<strong>Input:</strong>
[
  [0,1,0],
  [0,0,1],
  [1,1,1],
  [0,0,0]
]
<strong>Output:</strong>
[
  [0,0,0],
  [1,0,1],
  [0,1,1],
  [0,1,0]
]
</pre>

#### Follow up:
1. Could you solve it in-place? Remember that the board needs to be updated at the same time: You cannot update some cells first and then use their updated values to update other cells.
2. In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches the border of the array. How would you address these problems?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        let nbs = [(-1, -1), (-1, 0), (-1, 1), (0, -1),
            (0, 1), (1, -1), (1, 0), (1, 1)];

        for i in 0..m {
            for j in 0..n {
                let mut cnt_1 = 0;
                for (r, c) in &nbs {
                    if i >= -r && i + r < m && j >= -c && j + c < n {
                        cnt_1 += board[(i + r) as usize][(j + c) as usize] % 2;
                    }
                }

                match (board[i as usize][j as usize], cnt_1) {
                    (0, 3) => board[i as usize][j as usize] = 2,
                    (1, x) if x < 2 || x > 3 => board[i as usize][j as usize] = 3,
                    _ => (),
                }
            }
        }

        for i in 0..(m as usize) {
            for j in 0..(n as usize) {
                match board[i][j] {
                    2 => board[i][j] = 1,
                    3 => board[i][j] = 0,
                    _ => (),
                }
            }
        }
    }
}
```
