# 848. Shifting Letters
We have a string `S` of lowercase letters, and an integer array `shifts`.

Call the *shift* of a letter, the next letter in the alphabet, (wrapping around so that `'z'` becomes `'a'`).

For example, `shift('a') = 'b'`, `shift('t') = 'u'`, and `shift('z') = 'a'`.

Now for each `shifts[i] = x`, we want to shift the first `i+1` letters of `S`, `x` times.

Return the final string after all such shifts to `S` are applied.

#### Example 1:
<pre>
<strong>Input:</strong> S = "abc", shifts = [3,5,9]
<strong>Output:</strong> "rpl"
<strong>Explanation:</strong>
We start with "abc".
After shifting the first 1 letters of S by 3, we have "dbc".
After shifting the first 2 letters of S by 5, we have "igc".
After shifting the first 3 letters of S by 9, we have "rpl", the answer.
</pre>

#### Note:
1. `1 <= S.length = shifts.length <= 20000`
2. `0 <= shifts[i] <= 10 ^ 9`

## Solutions (Rust)

### 1. Prefix Sum
```Rust
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut s = s.into_bytes();
        let mut shift = 0;

        for i in (0..s.len()).rev() {
            shift = (shift + shifts[i]) % 26;
            s[i] = (s[i] - b'a' + (shift as u8)) % 26 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}
```
