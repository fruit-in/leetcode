# 639. Decode Ways II
A message containing letters from `A-Z` can be **encoded** into numbers using the following mapping:

```
'A' -> "1"
'B' -> "2"
...
'Z' -> "26"
```

To **decode** an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, `"11106"` can be mapped into:

* `"AAJF"` with the grouping `(1 1 10 6)`
* `"KJF"` with the grouping `(11 10 6)`

Note that the grouping `(1 11 06)` is invalid because `"06"` cannot be mapped into `'F'` since `"6"` is different from `"06"`.

**In addition** to the mapping above, an encoded message may contain the `'*'` character, which can represent any digit from `'1'` to `'9'` (`'0'` is excluded). For example, the encoded message `"1*"` may represent any of the encoded messages `"11"`, `"12"`, `"13"`, `"14"`, `"15"`, `"16"`, `"17"`, `"18"`, or `"19"`. Decoding `"1*"` is equivalent to decoding **any** of the encoded messages it can represent.

Given a string `s` consisting of digits and `'*'` characters, return *the **number** of ways to **decode** it*.

Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "*"
<strong>Output:</strong> 9
<strong>Explanation:</strong> The encoded message can represent any of the encoded messages "1", "2", "3", "4", "5", "6", "7", "8", or "9".
Each of these can be decoded to the strings "A", "B", "C", "D", "E", "F", "G", "H", and "I" respectively.
Hence, there are a total of 9 ways to decode "*".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "1*"
<strong>Output:</strong> 18
<strong>Explanation:</strong> The encoded message can represent any of the encoded messages "11", "12", "13", "14", "15", "16", "17", "18", or "19".
Each of these encoded messages have 2 ways to be decoded (e.g. "11" can be decoded to "AA" or "K").
Hence, there are a total of 9 * 2 = 18 ways to decode "1*".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "2*"
<strong>Output:</strong> 15
<strong>Explanation:</strong> The encoded message can represent any of the encoded messages "21", "22", "23", "24", "25", "26", "27", "28", or "29".
"21", "22", "23", "24", "25", and "26" have 2 ways of being decoded, but "27", "28", and "29" only have 1 way.
Hence, there are a total of (6 * 2) + (3 * 1) = 12 + 3 = 15 ways to decode "2*".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is a digit or `'*'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![[0_i64; 2]; s.len()];

        match s[0] {
            b'0' => return 0,
            b'*' => dp[0][0] = 9,
            _ => dp[0][0] = 1,
        }

        if s.len() > 1 {
            dp[1] = match (s[0], s[1]) {
                (b'1', b'0') => [0, 1],
                (b'1', b'*') => [9, 9],
                (b'1', _) => [1, 1],
                (b'2', b'0') => [0, 1],
                (b'2', b'*') => [9, 6],
                (b'2', x) if x < b'7' => [1, 1],
                (b'2', _) => [1, 0],
                (b'*', b'0') => [0, 2],
                (b'*', b'*') => [81, 15],
                (b'*', x) if x < b'7' => [9, 2],
                (b'*', _) => [9, 1],
                (_, b'0') => [0, 0],
                (_, b'*') => [9, 0],
                _ => [1, 0],
            };
        }

        for i in 2..s.len() {
            dp[i][0] = match s[i] {
                b'0' => 0,
                b'*' => 9 * (dp[i - 1][0] + dp[i - 1][1]),
                _ => dp[i - 1][0] + dp[i - 1][1],
            } % 1_000_000_007;

            dp[i][1] = match (s[i - 1], s[i]) {
                (b'1', b'*') => 9 * (dp[i - 2][0] + dp[i - 2][1]),
                (b'1', _) => dp[i - 2][0] + dp[i - 2][1],
                (b'2', b'*') => 6 * (dp[i - 2][0] + dp[i - 2][1]),
                (b'2', x) if x < b'7' => dp[i - 2][0] + dp[i - 2][1],
                (b'*', b'*') => 15 * (dp[i - 2][0] + dp[i - 2][1]),
                (b'*', x) if x < b'7' => 2 * (dp[i - 2][0] + dp[i - 2][1]),
                (b'*', _) => dp[i - 2][0] + dp[i - 2][1],
                _ => 0,
            } % 1_000_000_007;
        }

        ((dp[dp.len() - 1][0] + dp[dp.len() - 1][1]) % 1_000_000_007) as i32
    }
}
```
