# 130. 被围绕的区域
给你一个 `m x n` 的矩阵 `board` ，由若干字符 `'X'` 和 `'O'` 组成，**捕获** 所有 **被围绕的区域**：
* **连接：**一个单元格与水平或垂直方向上相邻的单元格连接。
* **区域：**连接所有 `'O'` 的单元格来形成一个区域。
* **围绕：**如果您可以用 `'X'` 单元格 **连接这个区域**，并且区域中没有任何单元格位于 `board` 边缘，则该区域被 `'X'` 单元格围绕。

通过 **原地** 将输入矩阵中的所有 `'O'` 替换为 `'X'` 来 **捕获被围绕的区域**。你不需要返回任何值。

#### 示例 1:
<pre>
<strong>输入:</strong> board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
<strong>输出:</strong> [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
<strong>解释:</strong>
<img src=https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg>
在上图中，底部的区域没有被捕获，因为它在 board 的边缘并且不能被围绕。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> board = [["X"]]
<strong>输出:</strong> [["X"]]
</pre>

#### 提示:
* `m == board.length`
* `n == board[i].length`
* `1 <= m, n <= 200`
* `board[i][j]` 为 `'X'` 或 `'O'`

## 题解 (Rust)

### 1. 题解
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
