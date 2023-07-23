# 1347. Minimum Number of Steps to Make Two Strings Anagram
Given two equal-size strings `s` and `t`. In one step you can choose **any character** of `t` and replace it with **another character**.

Return *the minimum number of steps* to make `t` an anagram of `s`.

An **Anagram** of a string is a string that contains the same characters with a different (or the same) ordering.

#### Example 1:
<pre>
<b>Input:</b> s = "bab", t = "aba"
<b>Output:</b> 1
<b>Explanation:</b> Replace the first 'a' in t with b, t = "bba" which is anagram of s.
</pre>

#### Example 2:
<pre>
<b>Input:</b> s = "leetcode", t = "practice"
<b>Output:</b> 5
<b>Explanation:</b> Replace 'p', 'r', 'a', 'i' and 'c' from t with proper characters to make t anagram of s.
</pre>

#### Example 3:
<pre>
<b>Input:</b> s = "anagram", t = "mangaar"
<b>Output:</b> 0
<b>Explanation:</b> "anagram" and "mangaar" are anagrams.
</pre>

#### Example 4:
<pre>
<b>Input:</b> s = "xxyyzz", t = "xxyyzz"
<b>Output:</b> 0
</pre>

#### Example 5:
<pre>
<b>Input:</b> s = "friend", t = "family"
<b>Output:</b> 4
</pre>

#### Constraints:
* `1 <= s.length <= 50000`
* `s.length == t.length`
* `s` and `t` contain lower-case English letters only.

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut cnt = [0; 26];

        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
            cnt[(t[i] - b'a') as usize] -= 1;
        }

        cnt.iter().filter(|&&x| x > 0).sum()
    }
}
```
