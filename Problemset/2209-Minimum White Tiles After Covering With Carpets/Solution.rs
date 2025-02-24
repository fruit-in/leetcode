impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let num_carpets = num_carpets as usize;
        let carpet_len = carpet_len as usize;
        let floor = floor.as_bytes();
        let mut prefix_sum = vec![0; floor.len() + 1];
        let mut dp = vec![vec![i32::MAX; num_carpets + 1]; floor.len() + 1];

        for i in 0..floor.len() {
            prefix_sum[i + 1] = prefix_sum[i] + (floor[i] - b'0') as i32;
        }

        for i in 0..floor.len() {
            dp[i][0] = *prefix_sum.last().unwrap();
            for j in 0..num_carpets {
                let k = floor.len().min(i + carpet_len);
                dp[k][j + 1] = dp[k][j + 1].min(dp[i][j] - prefix_sum[k] + prefix_sum[i]);
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            }
            dp[i + 1][num_carpets] = dp[i + 1][num_carpets].min(dp[i][num_carpets]);
        }

        *dp[floor.len()].iter().min().unwrap()
    }
}
