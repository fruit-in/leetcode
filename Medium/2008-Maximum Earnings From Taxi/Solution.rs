impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        let mut i = 1;
        let mut dp = vec![0; n as usize + 1];
        rides.sort_unstable();

        for ride in rides {
            let (start, end, tip) = (ride[0], ride[1], ride[2]);

            for j in i..=start as usize {
                dp[j] = dp[j].max(dp[j - 1]);
            }

            i = start as usize + 1;
            dp[end as usize] =
                dp[end as usize].max(dp[start as usize] + (end - start + tip) as i64);
        }

        for j in i..=n as usize {
            dp[j] = dp[j].max(dp[j - 1]);
        }

        dp[n as usize]
    }
}
