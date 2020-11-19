# 1573. Number of Ways to Split a String
Given a binary string `s` (a string consisting only of '0's and '1's), we can split `s` into 3 **non-empty** strings s1, s2, s3 (s1+ s2+ s3 = s).

Return the number of ways `s` can be split such that the number of characters '1' is the same in s1, s2, and s3.

Since the answer may be too large, return it modulo 10^9 + 7.

#### Example 1:
<pre>
<b>Input:</b> s = "10101"
<b>Output:</b> 4
<b>Explanation:</b> There are four ways to split s in 3 parts where each part contain the same number of letters '1'.
"1|010|1"
"1|01|01"
"10|10|1"
"10|1|01"
</pre>

#### Example 2:
<pre>
<b>Input:</b> s = "1001"
<b>Output:</b> 0
</pre>

#### Example 3:
<pre>
<b>Input:</b> s = "0000"
<b>Output:</b> 3
<b>Explanation:</b> There are three ways to split s in 3 parts.
"0|0|00"
"0|00|0"
"00|0|0"
</pre>

#### Example 4:
<pre>
<b>Input:</b> s = "100100010100110"
<b>Output:</b> 12
</pre>

#### Constraints:
* `3 <= s.length <= 10^5`
* `s[i]` is `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let mut indices = s
            .bytes()
            .enumerate()
            .filter(|&(i, c)| c == b'1')
            .map(|(i, c)| i as i32)
            .collect::<Vec<_>>();

        if indices.len() % 3 != 0 {
            return 0;
        }

        if indices.is_empty() {
            return ((s.len() - 2) as i64 * (s.len() - 1) as i64 / 2 % 1000000007) as i32;
        }

        let third = indices.len() / 3;
        let split_12 = (indices[third] - indices[third - 1]) as i64;
        let split_23 = (indices[2 * third] - indices[2 * third - 1]) as i64;

        (split_12 * split_23 % 1000000007) as i32
    }
}
```
