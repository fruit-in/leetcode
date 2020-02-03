impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; piles.len()]; piles.len()];

        for r in 1..piles.len() {
            for l in (0..r).rev() {
                dp[l][r] = match (r - l) % 2 {
                    1 => (piles[l] + dp[l + 1][r]).max(piles[r] + dp[l][r - 1]),
                    _ => (dp[l + 1][r]).min(dp[l][r - 1]),
                };
            }
        }

        dp[0].last().unwrap() * 2 > piles.iter().sum()
    }
}
