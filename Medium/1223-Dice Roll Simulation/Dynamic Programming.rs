impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        const M: i32 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; 16]; 7]; n as usize];
        (1..=6).for_each(|j| dp[0][j][0] = 1);
        let mut ret = 0;

        for i in 1..(n as usize) {
            for j in 1..=6 {
                let f = |x: usize| {
                    dp[i - 1][x][..(roll_max[x - 1] as usize)]
                        .iter()
                        .fold(0, |acc, x| (acc + x) % M)
                };
                dp[i][j][0] = (1..=6)
                    .filter(|&x| x != j)
                    .map(|x| f(x))
                    .fold(0, |acc, x| (acc + x) % M);
                for k in 1..(roll_max[j - 1] as usize).min(i + 1) {
                    dp[i][j][k] = dp[i - 1][j][k - 1];
                }
            }
        }

        for j in 1..=6 {
            for k in 0..(roll_max[j - 1].min(n) as usize) {
                ret = (ret + dp[n as usize - 1][j][k]) % M;
            }
        }

        ret
    }
}
