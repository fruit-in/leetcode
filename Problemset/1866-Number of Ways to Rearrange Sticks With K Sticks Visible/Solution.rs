impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let mut dp0 = vec![0; n as usize + 1];

        for i in 1..=k as usize {
            let mut dp1 = vec![0; n as usize + 1];
            dp1[i] = 1;

            for j in i + 1..=n as usize {
                dp1[j] = (dp1[j - 1] as i64 * (j as i64 - 1) % 1_000_000_007) as i32
                    + dp0[j - 1] % 1_000_000_007;
            }

            dp0 = dp1;
        }

        dp0[n as usize] % 1_000_000_007
    }
}
