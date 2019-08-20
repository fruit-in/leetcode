# 36. 有效的数独
判断一个 9x9 的数独是否有效。只需要**根据以下规则**，验证已经填入的数字是否有效即可。
1. 数字```1-9```在每一行只能出现一次。
2. 数字```1-9```在每一列只能出现一次。
3. 数字```1-9```在每一个以粗实线分隔的```3x3```宫内只能出现一次。

![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)<br>
上图是一个部分填充的有效的数独。

数独部分空格内已填入了数字，空白格用```'.'```表示。

#### 示例 1:
<pre>
<strong>输入:</strong>
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
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
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
<strong>输出:</strong> false
<strong>解释:</strong> 除了第一行的第一个数字从 <strong>5</strong> 改为 <strong>8</strong> 以外，空格内其他数字均与 示例1 相同。
但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
</pre>

#### 说明:
* 一个有效的数独（部分已被填充）不一定是可解的。
* 只需要根据以上规则，验证已经填入的数字是否有效即可。
* 给定数独序列只包含数字 ```1-9``` 和字符 ```'.'``` 。
* 给定数独永远是 ```9x9``` 形式的。

## 题解 (Rust)

### 1. 题解1
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

### 2. 题解2
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
