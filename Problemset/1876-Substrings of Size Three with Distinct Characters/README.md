# 1876. Substrings of Size Three with Distinct Characters
A string is **good** if there are no repeated characters.

Given a string `s`, return *the number of **good substrings** of length **three** in* `s`.

Note that if there are multiple occurrences of the same substring, every occurrence should be counted.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "xyzzaz"
<strong>Output:</strong> 1
<strong>Explanation:</strong> There are 4 substrings of size 3: "xyz", "yzz", "zza", and "zaz".
The only good substring of length 3 is "xyz".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aababcabc"
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 7 substrings of size 3: "aab", "aba", "bab", "abc", "bca", "cab", and "abc".
The good substrings are "abc", "bca", "cab", and "abc".
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes()
            .windows(3)
            .filter(|gs| gs[0] != gs[1] && gs[0] != gs[2] && gs[1] != gs[2])
            .count() as i32
    }
}
```
