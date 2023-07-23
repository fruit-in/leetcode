impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        if start_pos - k > end_pos || start_pos + k < end_pos {
            return 0;
        }

        let mut dp = vec![vec![0; 2 * k as usize + 1]; k as usize + 1];
        dp[0][k as usize] = 1;

        for i in 0..k as usize {
            for j in k as usize - i..=k as usize + i {
                dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % 1_000_000_007;
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % 1_000_000_007;
            }
        }

        dp[k as usize][(end_pos - start_pos + k) as usize]
    }
}
