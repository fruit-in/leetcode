# 1987. Number of Unique Good Subsequences
You are given a binary string `binary`. A **subsequence** of `binary` is considered **good** if it is **not empty** and has **no leading zeros** (with the exception of `"0"`).

Find the number of **unique good subsequences** of `binary`.

* For example, if `binary = "001"`, then all the **good** subsequences are `["0", "0", "1"]`, so the **unique** good subsequences are `"0"` and `"1"`. Note that subsequences `"00"`, `"01"`, and `"001"` are not good because they have leading zeros.

Return *the number of **unique good subsequences** of* `binary`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **subsequence** is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> binary = "001"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The good subsequences of binary are ["0", "0", "1"].
The unique good subsequences are "0" and "1".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> binary = "11"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The good subsequences of binary are ["1", "1", "11"].
The unique good subsequences are "1" and "11".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> binary = "101"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good subsequences of binary are ["1", "0", "1", "10", "11", "101"].
The unique good subsequences are "0", "1", "10", "11", and "101".
</pre>

#### Constraints:
* <code>1 <= binary.length <= 10<sup>5</sup></code>
* `binary` consists of only `'0'`s and `'1'`s.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = binary.len();
        let mut last_index = [n; 2];
        let mut first0 = false;
        let mut next = [false; 2];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, b) in binary.bytes().map(|b| (b - b'0') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[b]]).rem_euclid(MOD);
            last_index[b] = i;

            if next[b] {
                dp[i + 1] = (dp[i + 1] - 1).rem_euclid(MOD);
                next[b] = false;
            }
            if b == 0 && !first0 {
                first0 = true;
                next = [true; 2];
            }
        }

        *dp.last().unwrap()
    }
}
```
