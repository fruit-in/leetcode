# 2223. Sum of Scores of Built Strings
You are **building** a string `s` of length `n` **one** character at a time, **prepending** each new character to the **front** of the string. The strings are labeled from `1` to `n`, where the string with length `i` is labeled <code>s<sub>i</sub></code>.

* For example, for `s = "abaca"`, <code>s<sub>1</sub> == "a"</code>, <code>s<sub>2</sub> == "ca"</code>, <code>s<sub>3</sub> == "aca"</code>, etc.

The **score** of <code>s<sub>i</sub></code> is the length of the **longest common prefix** between <code>s<sub>i</sub></code> and <code>s<sub>n</sub></code> (Note that <code>s == s<sub>n</sub></code>).

Given the final string `s`, return *the **sum** of the **score** of every* <code>s<sub>i</sub></code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "babab"
<strong>Output:</strong> 9
<strong>Explanation:</strong>
For s1 == "b", the longest common prefix is "b" which has a score of 1.
For s2 == "ab", there is no common prefix so the score is 0.
For s3 == "bab", the longest common prefix is "bab" which has a score of 3.
For s4 == "abab", there is no common prefix so the score is 0.
For s5 == "babab", the longest common prefix is "babab" which has a score of 5.
The sum of the scores is 1 + 0 + 3 + 0 + 5 = 9, so we return 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "azbazbzaz"
<strong>Output:</strong> 14
<strong>Explanation:</strong>
For s2 == "az", the longest common prefix is "az" which has a score of 2.
For s6 == "azbzaz", the longest common prefix is "azb" which has a score of 3.
For s9 == "azbazbzaz", the longest common prefix is "azbazbzaz" which has a score of 9.
For all other si, the score is 0.
The sum of the scores is 2 + 3 + 9 = 14, so we return 14.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
