use std::collections::HashSet;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        const BASE: i64 = 257;
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        let mut rolling_hash1 = HashSet::new();
        let mut rolling_hash2 = HashSet::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            let mut count = 0;
            let mut hash1 = 0;
            let mut hash2 = 0;

            for j in i..nums.len() {
                if nums[j] % p == 0 {
                    count += 1;
                }
                if count > k {
                    break;
                }

                hash1 = (hash1 * BASE + nums[j] as i64) % MOD1;
                hash2 = (hash2 * BASE + nums[j] as i64) % MOD2;

                if !rolling_hash1.contains(&hash1) || !rolling_hash2.contains(&hash2) {
                    ret += 1;
                }

                rolling_hash1.insert(hash1);
                rolling_hash2.insert(hash2);
            }
        }

        ret
    }
}
