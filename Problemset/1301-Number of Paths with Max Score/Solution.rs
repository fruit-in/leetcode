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
