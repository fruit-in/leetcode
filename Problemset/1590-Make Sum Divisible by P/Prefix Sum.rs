use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let mut prefix = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut hash = HashMap::new();
        let mut ret = std::i32::MAX;

        for i in 1..prefix.len() {
            prefix[i] += prefix[i - 1];
        }

        let sum = *prefix.last().unwrap();

        for i in 0..prefix.len() {
            hash.insert(prefix[i] % p, i);
            if prefix[i] % p == 0 {
                ret = ret.min((prefix.len() - 1 - i) as i32);
            }
            if (sum - prefix[i]) % p == 0 {
                match hash.get(&0) {
                    Some(&j) => ret = ret.min((i - j) as i32),
                    None => ret = ret.min(i as i32 + 1),
                }
            }
            if let Some(&j) = hash.get(&(p - (sum - prefix[i]) % p)) {
                ret = ret.min((i - j) as i32);
            }
        }

        if ret == prefix.len() as i32 {
            return -1;
        }

        ret
    }
}
