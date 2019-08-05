# 36. Valid Sudoku
Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated **according to the following rules**:
1. Each row must contain the digits <code>1-9</code> without repetition.
2. Each column must contain the digits <code>1-9</code> without repetition.
3. Each of the 9 <code>3x3</code> sub-boxes of the grid must contain the digits <code>1-9</code> without repetition.

![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)<br>
A partially filled sudoku which is valid.

The Sudoku board could be partially filled, where empty cells are filled with the character <code>'.'</code>.

#### Example 1:
<pre>
<strong>Input:</strong>
[
  ["5","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
[
  ["8","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
<strong>Output:</strong> false
<strong>Explanation:</strong> Same as Example 1, except with the <strong>5</strong> in the top left corner being modified to <strong>8</strong>.
Since there are two 8's in the top left 3x3 sub-box, it is invalid.
</pre>

#### Note:
* A Sudoku board (partially filled) could be valid but is not necessarily solvable.
* Only the filled cells need to be validated according to the mentioned rules.
* The given board contain only digits <code>1-9</code> and the character <code>'.'</code>.
* The given board size is always <code>9x9</code>.

## Solutions

### 1. Solution1 (Rust)
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cnt_row;
        let mut set_row = HashSet::new();
        let mut cnt_col = vec![0; 9];
        let mut set_col = vec![HashSet::new(); 9];
        let mut cnt_sub = vec![0; 3];
        let mut set_sub = vec![HashSet::new(); 3];

        for row in 0..9 {
            cnt_row = 0;
            set_row.clear();
            if row % 3 == 0 {
                for i in 0..3 {
                    cnt_sub[i] = 0;
                    set_sub[i].clear();
                }
            }

            for col in 0..9 {
                if board[row][col] != '.' {
                    cnt_row += 1;
                    set_row.insert(board[row][col]);
                    cnt_col[col] += 1;
                    set_col[col].insert(board[row][col]);
                    cnt_sub[col / 3] += 1;
                    set_sub[col / 3].insert(board[row][col]);
                }

                if col % 3 == 2 {
                    if cnt_sub[col / 3] != set_sub[col / 3].len() {
                        return false;
                    }
                }
                if row == 8 {
                    if cnt_col[col] != set_col[col].len() {
                        return false;
                    }
                }
            }
            if cnt_row != set_row.len() {
                return false;
            }
        }
        true
    }
}
```

### 2. Solution2 (Rust)
```Rust
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut ans = vec![vec![vec![false; 9]; 3] ;9];
        for row in 0..9 {
            for col in 0..9 {
                let sub = row / 3 * 3 + col / 3;
                if board[row][col] != '.' {
                    let n = board[row][col].to_digit(10).unwrap() as usize - 1;
                    if ans[n][0][row] || ans[n][1][col] || ans[n][2][sub] {
                        return false;
                    }
                    ans[n][0][row] = true;
                    ans[n][1][col] = true;
                    ans[n][2][sub] = true;
                }
            }
        }
        true
    }
}
```
