impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        const BASE: i64 = 131;
        const MOD: i64 = 1_000_000_007;
        let s = s.as_bytes();
        let mut prefix_pow = vec![1; s.len()];
        let mut prefix_hash = vec![s[0] as i64; s.len()];
        let mut ret = s.len();

        for i in 1..s.len() {
            prefix_pow[i] = prefix_pow[i - 1] * BASE % MOD;
            prefix_hash[i] = (prefix_hash[i - 1] * BASE + s[i] as i64) % MOD;
        }

        for i in 1..s.len() {
            if s[i] != s[0] {
                continue;
            }

            let mut l = 1;
            let mut r = s.len() - i + 1;

            while l < r {
                let m = (l + r) / 2;
                let hash =
                    (prefix_hash[i + m - 1] - prefix_hash[i - 1] * prefix_pow[m]).rem_euclid(MOD);

                if hash == prefix_hash[m - 1] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }

            ret += l - 1;
        }

        ret as i64
    }
}
