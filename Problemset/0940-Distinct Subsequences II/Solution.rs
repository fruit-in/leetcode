impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let mut last_index = [n; 26];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, c) in s.bytes().map(|c| (c - b'a') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[c]]).rem_euclid(MOD);
            last_index[c] = i;
        }

        *dp.last().unwrap()
    }
}
