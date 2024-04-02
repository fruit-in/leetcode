# 424. Longest Repeating Character Replacement
You are given a string `s` and an integer `k`. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most `k` times.

Return *the length of the longest substring containing the same letter you can get after performing the above operations*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ABAB", k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Replace the two 'A's with two 'B's or vice versa.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "AABABBA", k = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> Replace the one 'A' in the middle with 'B' and form "AABBBBA".
The substring "BBBB" has the longest repeating letters, which is 4.
There may exists other ways to achieve this answer too.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only uppercase English letters.
* `0 <= k <= s.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 26];
        let mut most_ch = b'A';
        let mut i = -1;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'A') as usize] += 1;
            if count[(s[j] - b'A') as usize] > count[(most_ch - b'A') as usize] {
                most_ch = s[j];
            }
            while j as i32 - i - count[(most_ch - b'A') as usize] > k {
                i += 1;
                count[(s[i as usize] - b'A') as usize] -= 1;
                if s[i as usize] == most_ch {
                    most_ch = (0..26).max_by_key(|&i| count[i]).unwrap() as u8 + b'A';
                }
            }

            ret = ret.max(j as i32 - i);
        }

        ret
    }
}
```
