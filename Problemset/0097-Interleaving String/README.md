# 97. Interleaving String
Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an **interleaving** of `s1` and `s2`.

An **interleaving** of two strings `s` and `t` is a configuration where `s` and `t` are divided into `n` and `m` substrings respectively, such that:
* <code>s = s<sub>1</sub> + s<sub>2</sub> + ... + s<sub>n</sub></code>
* <code>t = t<sub>1</sub> + t<sub>2</sub> + ... + t<sub>m</sub></code>
* `|n - m| <= 1`
* The **interleaving** is <code>s<sub>1</sub> + t<sub>1</sub> + s<sub>2</sub> + t<sub>2</sub> + s<sub>3</sub> + t<sub>3</sub> + ...</code> or <code>t<sub>1</sub> + s<sub>1</sub> + t<sub>2</sub> + s<sub>2</sub> + t<sub>3</sub> + s<sub>3</sub> + ...</code>

**Note:** `a + b` is the concatenation of strings `a` and `b`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg)
<pre>
<strong>Input:</strong> s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
<strong>Output:</strong> true
<strong>Explanation:</strong> One way to obtain s3 is:
Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
Since s3 can be obtained by interleaving s1 and s2, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
<strong>Output:</strong> false
<strong>Explanation:</strong> Notice how it is impossible to interleave s2 with any other string to obtain s3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "", s2 = "", s3 = ""
<strong>Output:</strong> true
</pre>

#### Constraints:
* `0 <= s1.length, s2.length <= 100`
* `0 <= s3.length <= 200`
* `s1`, `s2`, and `s3` consist of lowercase English letters.

**Follow up:** Could you solve it using only `O(s2.length)` additional memory space?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len() + 1];

        for i in 0..=s1.len() {
            let mut tmp = vec![false; s2.len() + 1];
            tmp[0] = i == 0;

            for j in 0..=s2.len() {
                tmp[j] |= i > 0 && s1[i - 1] == s3[i + j - 1] && dp[j];
                tmp[j] |= j > 0 && s2[j - 1] == s3[i + j - 1] && tmp[j - 1];
            }

            dp = tmp;
        }

        dp[s2.len()]
    }
}
```
