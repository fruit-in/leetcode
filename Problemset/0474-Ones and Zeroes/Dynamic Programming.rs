impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

        for s in strs {
            let zeros = s.chars().filter(|&c| c == '0').count();
            let ones = s.chars().filter(|&c| c == '1').count();
            for i in (zeros..=(m as usize)).rev() {
                for j in (ones..=(n as usize)).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }

        dp[m as usize][n as usize]
    }
}
