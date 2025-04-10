impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let (start, finish) = (start as usize, finish as usize);
        let fuel = fuel as usize;
        let mut dp = vec![vec![0; fuel + 1]; locations.len()];
        dp[start][0] = 1;

        for i in 0..fuel {
            for j in 0..locations.len() {
                for k in 0..locations.len() {
                    let cost = (locations[j] - locations[k]).abs() as usize;

                    if j != k && i + cost <= fuel {
                        dp[k][i + cost] = (dp[k][i + cost] + dp[j][i]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[finish]
            .iter()
            .fold(0, |acc, x| (acc + x) % 1_000_000_007)
    }
}
