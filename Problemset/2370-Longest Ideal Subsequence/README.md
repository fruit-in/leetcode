# 2370. Longest Ideal Subsequence
You are given a string `s` consisting of lowercase letters and an integer `k`. We call a string `t` **ideal** if the following conditions are satisfied:
* `t` is a **subsequence** of the string `s`.
* The absolute difference in the alphabet order of every two **adjacent** letters in `t` is less than or equal to `k`.

Return *the length of the **longest** ideal string*.

A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

**Note** that the alphabet order is not cyclic. For example, the absolute difference in the alphabet order of `'a'` and `'z'` is `25`, not `1`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "acfgbd", k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcd", k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest ideal string is "abcd". The length of this string is 4, so 4 is returned.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `0 <= k <= 25`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as u8;
        let mut dp = [0; 26];

        for c in s.bytes() {
            dp[(c - b'a') as usize] = (b'a'.max(c - k)..=b'z'.min(c + k))
                .map(|i| dp[(i - b'a') as usize])
                .max()
                .unwrap()
                + 1;
        }

        *dp.iter().max().unwrap()
    }
}
```
