# 2575. Find the Divisibility Array of a String
You are given a **0-indexed** string `word` of length `n` consisting of digits, and a positive integer `m`.

The **divisibility array** `div` of `word` is an integer array of length `n` such that:
* `div[i] = 1` if the **numeric value** of `word[0,...,i]` is divisible by `m`, or
* `div[i] = 0` otherwise.

Return *the divisibility array of* `word`.

#### Example 1:
<pre>
<strong>Input:</strong> word = "998244353", m = 3
<strong>Output:</strong> [1,1,0,0,0,1,1,0,0]
<strong>Explanation:</strong> There are only 4 prefixes that are divisible by 3: "9", "99", "998244", and "9982443".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "1010", m = 10
<strong>Output:</strong> [0,1,0,1]
<strong>Explanation:</strong> There are only 2 prefixes that are divisible by 10: "10", and "1010".
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* `word.length == n`
* `word` consists of digits from `0` to `9`
* <code>1 <= m <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let m = m as i64;
        let mut x = 0;
        let mut div = vec![0; word.len()];

        for (i, digit) in word.bytes().enumerate() {
            x = (x * 10 + (digit - b'0') as i64) % m;
            div[i] = 0.max(1 - x as i32);
        }

        div
    }
}
```
