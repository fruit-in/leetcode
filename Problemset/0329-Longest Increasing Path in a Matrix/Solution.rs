impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut cells = vec![];
        let mut dp = vec![vec![1; n]; m];
        let mut ret = 1;

        for r in 0..m {
            for c in 0..n {
                cells.push((r, c));
            }
        }

        cells.sort_unstable_by_key(|&(r, c)| matrix[r][c]);

        for (r, c) in cells {
            let mut max_path = 0;

            if r > 0 && matrix[r - 1][c] < matrix[r][c] {
                max_path = max_path.max(dp[r - 1][c]);
            }
            if r < m - 1 && matrix[r + 1][c] < matrix[r][c] {
                max_path = max_path.max(dp[r + 1][c]);
            }
            if c > 0 && matrix[r][c - 1] < matrix[r][c] {
                max_path = max_path.max(dp[r][c - 1]);
            }
            if c < n - 1 && matrix[r][c + 1] < matrix[r][c] {
                max_path = max_path.max(dp[r][c + 1]);
            }

            dp[r][c] = max_path + 1;
            ret = ret.max(dp[r][c]);
        }

        ret
    }
}
