# 2430. Maximum Deletions on a String
You are given a string `s` consisting of only lowercase English letters. In one operation, you can:
* Delete **the entire string** `s`, or
* Delete the **first** `i` letters of `s` if the first `i` letters of `s` are **equal** to the following `i` letters in `s`, for any `i` in the range `1 <= i <= s.length / 2`.

For example, if `s = "ababc"`, then in one operation, you could delete the first two letters of `s` to get `"abc"`, since the first two letters of `s` and the following two letters of `s` are both equal to `"ab"`.

Return *the **maximum** number of operations needed to delete all of* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcabcdabc"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- Delete the first 3 letters ("abc") since the next 3 letters are equal. Now, s = "abcdabc".
- Delete all the letters.
We used 2 operations so return 2. It can be proven that 2 is the maximum number of operations needed.
Note that in the second operation we cannot delete "abc" again because the next occurrence of "abc" does not happen in the next 3 letters.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaabaab"
<strong>Output:</strong> 4
<strong>Explanation:</strong>
- Delete the first letter ("a") since the next letter is equal. Now, s = "aabaab".
- Delete the first 3 letters ("aab") since the next 3 letters are equal. Now, s = "aab".
- Delete the first letter ("a") since the next letter is equal. Now, s = "ab".
- Delete all the letters.
We used 4 operations so return 4. It can be proven that 4 is the maximum number of operations needed.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aaaaa"
<strong>Output:</strong> 5
<strong>Explanation:</strong> In each operation, we can delete the first letter of s.
</pre>

#### Constraints:
* `1 <= s.length <= 4000`
* `s` consists only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0; n]; n];
        let mut dp = vec![1; n];

        for i in (0..n - 1).rev() {
            for j in (i + 1..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = 1;
                    if j + 1 < n {
                        lcp[i][j] += lcp[i + 1][j + 1];
                    }
                }
            }
        }

        for i in (0..n - 1).rev() {
            for j in i + 1..=(n + i) / 2 {
                if lcp[i][j] >= j - i {
                    dp[i] = dp[i].max(1 + dp[j]);
                }
            }
        }

        dp[0]
    }
}
```
