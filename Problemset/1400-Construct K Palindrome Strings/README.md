# 1400. Construct K Palindrome Strings
Given a string `s` and an integer `k`. You should construct `k` non-empty **palindrome** strings using **all the characters** in `s`.

Return ***True*** if you can use all the characters in `s` to construct `k` palindrome strings or ***False*** otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> s = "annabelle", k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> You can construct two palindromes using all characters in s.
Some possible constructions "anna" + "elble", "anbna" + "elle", "anellena" + "b"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "leetcode", k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to construct 3 palindromes using all the characters of s.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "true", k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> The only possible solution is to put each character in a separate string.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "yzyzyzyzyzyzyzy", k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> Simply you can put all z's in one string and all y's in the other string. Both strings will be palindrome.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "cr", k = 7
<strong>Output:</strong> false
<strong>Explanation:</strong> We don't have enough characters in s to construct 7 palindromes.
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* All characters in `s` are lower-case English letters.
* `1 <= k <= 10^5`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let count = s.bytes().fold(0i32, |acc, x| acc ^ (1 << (x - b'a')));

        count.count_ones() as i32 <= k && s.len() as i32 >= k
    }
}
```
