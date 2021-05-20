impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp0 = vec![vec![0; n as usize]; m as usize];
        dp0[start_row as usize][start_column as usize] = 1;
        let mut ret = 0;

        for _ in 0..max_move {
            let mut dp1 = vec![vec![0; n as usize]; m as usize];

            for r in 0..m as usize {
                for c in 0..n as usize {
                    if r > 0 {
                        dp1[r - 1][c] = (dp1[r - 1][c] + dp0[r][c]) % 1_000_000_007;
                    } else {
                        ret = (ret + dp0[r][c]) % 1_000_000_007;
                    }
                    if c > 0 {
                        dp1[r][c - 1] = (dp1[r][c - 1] + dp0[r][c]) % 1_000_000_007;
                    } else {
                        ret = (ret + dp0[r][c]) % 1_000_000_007;
                    }
                    if r + 1 < m as usize {
                        dp1[r + 1][c] = (dp1[r + 1][c] + dp0[r][c]) % 1_000_000_007;
                    } else {
                        ret = (ret + dp0[r][c]) % 1_000_000_007;
                    }
                    if c + 1 < n as usize {
                        dp1[r][c + 1] = (dp1[r][c + 1] + dp0[r][c]) % 1_000_000_007;
                    } else {
                        ret = (ret + dp0[r][c]) % 1_000_000_007;
                    }
                }
            }

            dp0 = dp1;
        }

        ret
    }
}
