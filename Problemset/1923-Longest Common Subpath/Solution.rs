use std::collections::HashMap;

impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        const BASE1: i64 = 100003;
        const MOD1: i64 = 1_000_000_007;
        const BASE2: i64 = 131071;
        const MOD2: i64 = 1_000_000_009;
        let max_len = paths.iter().map(|p| p.len()).max().unwrap();
        let min_len = paths.iter().map(|p| p.len()).min().unwrap();
        let mut base_pow1 = vec![1; max_len + 1];
        let mut base_pow2 = vec![1; max_len + 1];
        let mut l = 1;
        let mut r = min_len + 1;

        for i in 0..max_len {
            base_pow1[i + 1] = base_pow1[i] * BASE1 % MOD1;
            base_pow2[i + 1] = base_pow2[i] * BASE2 % MOD2;
        }

        while l < r {
            let m = (l + r) / 2;
            let mut count = HashMap::new();
            let mut flag = false;

            for i in 0..paths.len() {
                let mut hash1 = 0;
                let mut hash2 = 0;

                for j in 0..paths[i].len() {
                    hash1 = (hash1 * BASE1 + paths[i][j] as i64) % MOD1;
                    hash2 = (hash2 * BASE2 + paths[i][j] as i64) % MOD2;

                    if j >= m {
                        hash1 = (hash1 - paths[i][j - m] as i64 * base_pow1[m]).rem_euclid(MOD1);
                        hash2 = (hash2 - paths[i][j - m] as i64 * base_pow2[m]).rem_euclid(MOD2);
                    }
                    if j >= m - 1 {
                        if *count.get(&(hash1, hash2)).unwrap_or(&0) == i {
                            if i == paths.len() - 1 {
                                flag = true;
                                break;
                            }
                            count.insert((hash1, hash2), i + 1);
                        }
                    }
                }
            }

            if flag {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32 - 1
    }
}
