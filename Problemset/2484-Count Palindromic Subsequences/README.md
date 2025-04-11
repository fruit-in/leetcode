# 2484. Count Palindromic Subsequences
Given a string of digits `s`, return *the number of **palindromic subsequences** of* `s` *having length* `5`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Note:
* A string is **palindromic** if it reads the same forward and backward.
* A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

#### Example 1:
<pre>
<strong>Input:</strong> s = "103301"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
There are 6 possible subsequences of length 5: "10330","10331","10301","10301","13301","03301".
Two of them (both equal to "10301") are palindromic.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "0000000"
<strong>Output:</strong> 21
<strong>Explanation:</strong> All 21 subsequences are "00000", which is palindromic.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "9999900000"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only two palindromic subsequences are "99999" and "00000".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` consists of digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = s.bytes().map(|c| (c - b'0') as usize).collect::<Vec<_>>();
        let mut count1 = [0_i64; 10];
        let mut count2l = vec![[0; 100]; s.len()];
        let mut count2r = vec![[0; 100]; s.len()];
        let mut ret = 0;

        count1[s[0]] = 1;
        for i in 1..s.len() {
            count2l[i] = count2l[i - 1];
            for j in 0..10 {
                count2l[i][s[i] * 10 + j] = (count2l[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        count1 = [0; 10];
        count1[s[s.len() - 1]] = 1;
        for i in (0..s.len() - 1).rev() {
            count2r[i] = count2r[i + 1];
            for j in 0..10 {
                count2r[i][s[i] * 10 + j] = (count2r[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        for i in 2..s.len().saturating_sub(2) {
            for j in 0..100 {
                ret = (ret + count2l[i - 1][j] * count2r[i + 1][j] % 1_000_000_007) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
