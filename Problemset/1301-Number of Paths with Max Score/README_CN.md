# 1301. 最大得分的路径数目
给你一个正方形字符数组 `board` ，你从数组最右下方的字符 `'S'` 出发。

你的目标是到达数组最左上角的字符 `'E'` ，数组剩余的部分为数字字符 `1, 2, ..., 9` 或者障碍 `'X'`。在每一步移动中，你可以向上、向左或者左上方移动，可以移动的前提是到达的格子没有障碍。

一条路径的 「得分」 定义为：路径上所有数字的和。

请你返回一个列表，包含两个整数：第一个整数是 「得分」 的最大值，第二个整数是得到最大得分的方案数，请把结果对 **`10^9 + 7` 取余**。

如果没有任何路径可以到达终点，请返回 `[0, 0]` 。

#### 示例 1:
<pre>
<strong>输入:</strong> board = ["E23","2X2","12S"]
<strong>输出:</strong> [7,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> board = ["E12","1X1","21S"]
<strong>输出:</strong> [4,2]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> board = ["E11","XXX","11S"]
<strong>输出:</strong> [0,0]
</pre>

#### 提示:
* `2 <= board.length == board[i].length <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let mut board = board
            .into_iter()
            .map(|row| row.into_bytes())
            .collect::<Vec<_>>();
        let n = board.len();
        let mut dp = vec![vec![[-1, -1]; n]; n];
        board[0][0] = b'0';
        dp[n - 1][n - 1] = [0, 1];

        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if board[i][j] == b'X' || dp[i][j][1] < 0 {
                    continue;
                }

                if i > 0 && board[i - 1][j] != b'X' {
                    if dp[i - 1][j][0] < dp[i][j][0] + (board[i - 1][j] - b'0') as i32 {
                        dp[i - 1][j][0] = dp[i][j][0] + (board[i - 1][j] - b'0') as i32;
                        dp[i - 1][j][1] = 0;
                    }
                    if dp[i - 1][j][0] == dp[i][j][0] + (board[i - 1][j] - b'0') as i32 {
                        dp[i - 1][j][1] = (dp[i - 1][j][1] + dp[i][j][1]) % 1_000_000_007;
                    }
                }
                if j > 0 && board[i][j - 1] != b'X' {
                    if dp[i][j - 1][0] < dp[i][j][0] + (board[i][j - 1] - b'0') as i32 {
                        dp[i][j - 1][0] = dp[i][j][0] + (board[i][j - 1] - b'0') as i32;
                        dp[i][j - 1][1] = 0;
                    }
                    if dp[i][j - 1][0] == dp[i][j][0] + (board[i][j - 1] - b'0') as i32 {
                        dp[i][j - 1][1] = (dp[i][j - 1][1] + dp[i][j][1]) % 1_000_000_007;
                    }
                }
                if i > 0 && j > 0 && board[i - 1][j - 1] != b'X' {
                    if dp[i - 1][j - 1][0] < dp[i][j][0] + (board[i - 1][j - 1] - b'0') as i32 {
                        dp[i - 1][j - 1][0] = dp[i][j][0] + (board[i - 1][j - 1] - b'0') as i32;
                        dp[i - 1][j - 1][1] = 0;
                    }
                    if dp[i - 1][j - 1][0] == dp[i][j][0] + (board[i - 1][j - 1] - b'0') as i32 {
                        dp[i - 1][j - 1][1] = (dp[i - 1][j - 1][1] + dp[i][j][1]) % 1_000_000_007;
                    }
                }
            }
        }

        if dp[0][0][1] < 0 {
            return vec![0, 0];
        }

        dp[0][0].to_vec()
    }
}
```
