# 1461. Check If a String Contains All Binary Codes of Size K
Given a binary string `s` and an integer `k`.

Return `true` *if every binary code of length* `k` *is a substring of* `s`. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "00110110", k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indicies 0, 1, 3 and 2 respectively.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "00110", k = 2
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "0110", k = 1
<strong>Output:</strong> true
<strong>Explanation:</strong> The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "0110", k = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> The binary code "00" is of length 2 and doesn't exist in the array.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "0000000001011100", k = 4
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>1 <= s.length <= 5 * 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.
* `1 <= k <= 20`

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut x = i32::from_str_radix(&s[..k as usize], 2).unwrap();
        let y = 1 << k;
        let mut set = vec![x].into_iter().collect::<HashSet<_>>();

        for c in s.bytes().skip(k as usize) {
            x = (x << 1) % y + (c - b'0') as i32;
            set.insert(x);
        }

        set.len() == y as usize
    }
}
```
