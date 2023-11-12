impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for i in 0..types.len() {
            for j in (0..target).rev() {
                for k in 1..=types[i][0] {
                    let points = (k * types[i][1]) as usize + j;

                    if points > target {
                        break;
                    }

                    dp[points] = (dp[points] + dp[j]) % 1_000_000_007;
                }
            }
        }

        *dp.last().unwrap()
    }
}
