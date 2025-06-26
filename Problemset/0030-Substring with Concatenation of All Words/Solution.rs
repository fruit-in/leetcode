use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() < words.len() * words[0].len() {
            return vec![];
        }

        const BASE1: i64 = 131;
        const BASE2: i64 = 257;
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        let s = s.as_bytes();
        let m = s.len();
        let n = words[0].len();
        let concat_len = words.len() * n;
        let mut hash1 = 0;
        let mut hash2 = 0;
        let mut base_pow1 = (0..n).fold(1, |acc, _| acc * BASE1 % MOD1);
        let mut base_pow2 = (0..n).fold(1, |acc, _| acc * BASE2 % MOD2);
        let mut s_sub_hash = vec![(0, 0); m - n + 1];
        let mut words_hash_count = HashMap::new();
        let mut ret = vec![];

        for i in 0..m {
            hash1 = (hash1 * BASE1 + s[i] as i64) % MOD1;
            hash2 = (hash2 * BASE2 + s[i] as i64) % MOD2;

            if i >= n {
                hash1 = (hash1 - s[i - n] as i64 * base_pow1).rem_euclid(MOD1);
                hash2 = (hash2 - s[i - n] as i64 * base_pow2).rem_euclid(MOD2);
            }
            if i >= n - 1 {
                s_sub_hash[i + 1 - n] = (hash1, hash2);
            }
        }

        for word in &words {
            hash1 = 0;
            hash2 = 0;

            for c in word.bytes() {
                hash1 = (hash1 * BASE1 + c as i64) % MOD1;
                hash2 = (hash2 * BASE2 + c as i64) % MOD2;
            }

            *words_hash_count.entry((hash1, hash2)).or_insert(0) += 1;
        }

        for i in 0..n {
            let mut s_sub_hash_count = HashMap::new();
            let mut distinct_count = 0;
            let mut hash = (0, 0);

            for j in (i..=m - n).step_by(n) {
                if j - i >= concat_len {
                    hash = s_sub_hash[j - concat_len];
                    if s_sub_hash_count[&hash] == *words_hash_count.get(&hash).unwrap_or(&0) {
                        distinct_count -= 1;
                    }
                    *s_sub_hash_count.get_mut(&hash).unwrap() -= 1;
                }
                hash = s_sub_hash[j];
                *s_sub_hash_count.entry(hash).or_insert(0) += 1;
                if s_sub_hash_count[&hash] == *words_hash_count.get(&hash).unwrap_or(&0) {
                    distinct_count += 1;
                }

                if distinct_count == words_hash_count.len() {
                    ret.push((j + n - concat_len) as i32);
                }
            }
        }

        ret
    }
}
