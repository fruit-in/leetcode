impl Solution {
    const BANS: [[usize; 4]; 6] = [
        [0, 0, 0, 0],
        [1, 3, 5, 5],
        [2, 5, 5, 5],
        [1, 3, 5, 5],
        [4, 4, 4, 4],
        [1, 2, 3, 5],
    ];
    const MOD: i32 = 1_000_000_007;

    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }

        let mut dp = [[0; 6]; 6];
        let mut ret = 0;

        for i in 0..6 {
            for j in 0..6 {
                dp[i][j] = !Self::BANS[j].contains(&i) as i32;
            }
        }

        for _ in 2..n {
            let mut tmp = [[0; 6]; 6];

            for i in 0..6 {
                for j in 0..6 {
                    for k in (0..6).filter(|&x| x != i && !Self::BANS[x].contains(&j)) {
                        tmp[j][k] = (tmp[j][k] + dp[i][j]) % Self::MOD;
                    }
                }
            }

            dp = tmp;
        }

        for i in 0..6 {
            for j in 0..6 {
                ret = (ret + dp[i][j]) % Self::MOD;
            }
        }

        ret
    }
}
