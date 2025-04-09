impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target
            .bytes()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();
        let mut count = vec![[0; 26]; words[0].len()];
        let mut dp = vec![vec![0; count.len() + 1]; target.len() + 1];
        dp[0] = vec![1_i64; count.len() + 1];

        for word in &words {
            let word = word.as_bytes();

            for i in 0..word.len() {
                count[i][(word[i] - b'a') as usize] += 1;
            }
        }

        for i in 1..=target.len() {
            for j in 1..=count.len() {
                dp[i][j] = dp[i][j - 1];
                dp[i][j] =
                    (dp[i][j] + dp[i - 1][j - 1] * count[j - 1][target[i - 1]]) % 1_000_000_007;
            }
        }

        dp[target.len()][count.len()] as i32
    }
}
