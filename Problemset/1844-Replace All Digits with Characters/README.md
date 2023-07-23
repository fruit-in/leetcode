# 1844. Replace All Digits with Characters
You are given a **0-indexed** string `s` that has lowercase English letters in its **even** indices and digits in its **odd** indices.

There is a function `shift(c, x)`, where `c` is a character and `x` is a digit, that returns the <code>x<sup>th</sup></code> character after `c`.
* For example, `shift('a', 5) = 'f'` and `shift('x', 0) = 'x'`.

For every **odd** index `i`, you want to replace the digit `s[i]` with `shift(s[i-1], s[i])`.

Return `s` *after replacing all digits. It is **guaranteed** that* `shift(s[i-1], s[i])` *will never exceed* `'z'`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "a1c1e1"
<strong>Output:</strong> "abcdef"
<strong>Explanation:</strong> The digits are replaced as follows:
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('c',1) = 'd'
- s[5] -> shift('e',1) = 'f'
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "a1b2c3d4e"
<strong>Output:</strong> "abbdcfdhe"
<strong>Explanation:</strong> The digits are replaced as follows:
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('b',2) = 'd'
- s[5] -> shift('c',3) = 'f'
- s[7] -> shift('d',4) = 'h'
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` consists only of lowercase English letters and digits.
* `shift(s[i-1], s[i]) <= 'z'` for all **odd** indices `i`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();

        for i in (1..s.len()).step_by(2) {
            s[i] += s[i - 1] - b'0';
        }

        String::from_utf8(s).unwrap()
    }
}
```
