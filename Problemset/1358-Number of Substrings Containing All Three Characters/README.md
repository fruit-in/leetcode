# 1358. Number of Substrings Containing All Three Characters
Given a string `s` consisting only of characters *a*, *b* and *c*.

Return the number of substrings containing **at least** one occurrence of all these characters *a*, *b* and *c*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcabc"
<strong>Output:</strong> 10
<strong>Explanation:</strong> The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaacb"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "abc"
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `3 <= s.length <= 5 x 10^4`
* `s` only consists of *a*, *b* or *c* characters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 3];
        let mut i = 0;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'a') as usize] += 1;

            while count[0] * count[1] * count[2] > 0 {
                count[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }

            ret += i;
        }

        ret as i32
    }
}
```
