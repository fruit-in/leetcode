# 1044. Longest Duplicate Substring
Given a string `s`, consider all *duplicated substrings*: (contiguous) substrings of s that occur 2 or more times. The occurrences may overlap.

Return **any** duplicated substring that has the longest possible length. If `s` does not have a duplicated substring, the answer is `""`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "banana"
<strong>Output:</strong> "ana"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcd"
<strong>Output:</strong> ""
</pre>

#### Constraints:
* <code>2 <= s.length <= 3 * 10<sup>4</sup></code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        const BASE: i64 = 131;
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        let s = s.as_bytes();
        let mut l = 1;
        let mut r = s.len();
        let mut ret = String::new();

        while l < r {
            let m = (l + r) / 2;
            let mut hash1 = 0;
            let mut hash2 = 0;
            let mut rolling_hash1 = HashSet::new();
            let mut rolling_hash2 = HashSet::new();
            let mut flag = false;
            let mut base_pow_m1 = 1;
            let mut base_pow_m2 = 1;

            for _ in 0..m {
                base_pow_m1 = (base_pow_m1 * BASE) % MOD1;
                base_pow_m2 = (base_pow_m2 * BASE) % MOD2;
            }

            for i in 0..s.len() {
                hash1 = (hash1 * BASE + s[i] as i64) % MOD1;
                hash2 = (hash2 * BASE + s[i] as i64) % MOD2;
                if i >= m {
                    hash1 = (hash1 - s[i - m] as i64 * base_pow_m1).rem_euclid(MOD1);
                    hash2 = (hash2 - s[i - m] as i64 * base_pow_m2).rem_euclid(MOD2);
                    if rolling_hash1.contains(&hash1) && rolling_hash2.contains(&hash2) {
                        ret = String::from_utf8(s[i - m + 1..=i].to_vec()).unwrap();
                        flag = true;
                        break;
                    }
                }
                if i >= m - 1 {
                    rolling_hash1.insert(hash1);
                    rolling_hash2.insert(hash2);
                }
            }

            if flag {
                l = m + 1;
            } else {
                r = m;
            }
        }

        ret
    }
}
```
