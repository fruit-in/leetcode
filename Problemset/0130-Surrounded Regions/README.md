# 130. Surrounded Regions
You are given an `m x n` matrix `board` containing **letters** `'X'` and `'O'`, **capture regions** that are **surrounded**:
* **Connect:** A cell is connected to adjacent cells horizontally or vertically.
* **Region:** To form a region connect every `'O'` cell.
* **Surround:** The region is surrounded with `'X'` cells if you can **connect the region** with `'X'` cells and none of the region cells are on the edge of the `board`.

To capture a **surrounded region**, replace all `'O'`s with `'X'`s **in-place** within the original board. You do not need to return anything.

#### Example 1:
<pre>
<strong>Input:</strong> board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
<strong>Output:</strong> [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
<strong>Explanation:</strong>
<img src=https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg>
In the above diagram, the bottom region is not captured because it is on the edge of the board and cannot be surrounded.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> board = [["X"]]
<strong>Output:</strong> [["X"]]
</pre>

#### Constraints:
* `m == board.length`
* `n == board[i].length`
* `1 <= m, n <= 200`
* `board[i][j]` is `'X'` or `'O'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut stack = vec![];

        for i in 0..m {
            if board[i][0] == 'O' {
                board[i][0] = '0';
                stack.push((i, 0));
            }
            if board[i][n - 1] == 'O' {
                board[i][n - 1] = '0';
                stack.push((i, n - 1));
            }
        }
        for j in 0..n {
            if board[0][j] == 'O' {
                board[0][j] = '0';
                stack.push((0, j));
            }
            if board[m - 1][j] == 'O' {
                board[m - 1][j] = '0';
                stack.push((m - 1, j));
            }
        }

        while let Some((i, j)) = stack.pop() {
            if i > 0 && board[i - 1][j] == 'O' {
                board[i - 1][j] = '0';
                stack.push((i - 1, j));
            }
            if i < m - 1 && board[i + 1][j] == 'O' {
                board[i + 1][j] = '0';
                stack.push((i + 1, j));
            }
            if j > 0 && board[i][j - 1] == 'O' {
                board[i][j - 1] = '0';
                stack.push((i, j - 1));
            }
            if j < n - 1 && board[i][j + 1] == 'O' {
                board[i][j + 1] = '0';
                stack.push((i, j + 1));
            }
        }

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == '0' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
```
