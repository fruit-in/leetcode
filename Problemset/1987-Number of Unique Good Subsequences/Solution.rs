impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = binary.len();
        let mut last_index = [n; 2];
        let mut first0 = false;
        let mut next = [false; 2];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, b) in binary.bytes().map(|b| (b - b'0') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[b]]).rem_euclid(MOD);
            last_index[b] = i;

            if next[b] {
                dp[i + 1] = (dp[i + 1] - 1).rem_euclid(MOD);
                next[b] = false;
            }
            if b == 0 && !first0 {
                first0 = true;
                next = [true; 2];
            }
        }

        *dp.last().unwrap()
    }
}
