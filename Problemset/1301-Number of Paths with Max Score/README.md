# 1301. Number of Paths with Max Score
You are given a square `board` of characters. You can move on the board starting at the bottom right square marked with the character `'S'`.

You need to reach the top left square marked with the character `'E'`. The rest of the squares are labeled either with a numeric character `1, 2, ..., 9` or with an obstacle `'X'`. In one move you can go up, left or up-left (diagonally) only if there is no obstacle there.

Return a list of two integers: the first integer is the maximum sum of numeric characters you can collect, and the second is the number of such paths that you can take to get that maximum sum, **taken modulo `10^9 + 7`**.

In case there is no path, return `[0, 0]`.

#### Example 1:
<pre>
<strong>Input:</strong> board = ["E23","2X2","12S"]
<strong>Output:</strong> [7,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> board = ["E12","1X1","21S"]
<strong>Output:</strong> [4,2]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> board = ["E11","XXX","11S"]
<strong>Output:</strong> [0,0]
</pre>

#### Constraints:
* `2 <= board.length == board[i].length <= 100`

## Solutions (Rust)

### 1. Solution
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
