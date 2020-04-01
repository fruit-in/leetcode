impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;

        for &coin in &coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] = match dp[i - coin as usize] {
                    x if x != -1 && (dp[i] == -1 || dp[i] > x + 1) => x + 1,
                    _ => dp[i],
                };
            }
        }

        dp[amount as usize]
    }
}
