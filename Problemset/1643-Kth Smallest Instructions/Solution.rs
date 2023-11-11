impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let row = destination[0] as usize;
        let column = destination[1] as usize;
        let mut k = k as usize;
        let mut dp = vec![vec![1; row + 1]; row + column];
        let mut remainv = row;
        let mut remainh = column;
        let mut instructions = vec![];

        for n in 0..dp.len() {
            for m in 1..dp[0].len().min(n + 1) {
                dp[n][m] = dp[n][m - 1] * (n - m + 1) / m;
            }
        }

        for _ in 0..row + column {
            if remainh > 0 && k <= dp[remainv + remainh - 1][remainv] {
                remainh -= 1;
                instructions.push(b'H');
            } else {
                k -= dp[remainv + remainh - 1][remainv];
                remainv -= 1;
                instructions.push(b'V');
            }
        }

        String::from_utf8(instructions).unwrap()
    }
}
