impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in &coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] += dp[i - coin as usize];
            }
        }

        dp[amount as usize]
    }
}
