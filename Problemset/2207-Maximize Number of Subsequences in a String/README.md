# 2207. Maximize Number of Subsequences in a String
You are given a **0-indexed** string `text` and another **0-indexed** string `pattern` of length `2`, both of which consist of only lowercase English letters.

You can add **either** `pattern[0]` or `pattern[1]` anywhere in `text` **exactly once**. Note that the character can be added even at the beginning or at the end of `text`.

Return *the **maximum** number of times* `pattern` *can occur as a **subsequence** of the modified* `text`.

A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

#### Example 1:
<pre>
<strong>Input:</strong> text = "abdcdbc", pattern = "ac"
<strong>Output:</strong> 4
<strong>Explanation:</strong>
If we add pattern[0] = 'a' in between text[1] and text[2], we get "abadcdbc". Now, the number of times "ac" occurs as a subsequence is 4.
Some other strings which have 4 subsequences "ac" after adding a character to text are "aabdcdbc" and "abdacdbc".
However, strings such as "abdcadbc", "abdccdbc", and "abdcdbcc", although obtainable, have only 3 subsequences "ac" and are thus suboptimal.
It can be shown that it is not possible to get more than 4 subsequences "ac" by adding only one character.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "aabb", pattern = "ab
<strong>Output:</strong> 6
<strong>Explanation:</strong>
Some of the strings which can be obtained from text and have 6 subsequences "ab" are "aaabb", "aaabb", and "aabbb".
</pre>

#### Constraints:
* <code>1 <= text.length <= 10<sup>5</sup></code>
* `pattern.length == 2`
* `text` and `pattern` consist only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let text = text.as_bytes();
        let pattern = pattern.as_bytes();
        let n = text.len();
        let mut count0 = 1;
        let mut count1 = 1;
        let mut ret0 = 0;
        let mut ret1 = 0;

        for i in 0..text.len() {
            if text[i] == pattern[1] {
                ret0 += count0;
            }
            if text[i] == pattern[0] {
                count0 += 1;
            }

            if text[n - 1 - i] == pattern[0] {
                ret1 += count1;
            }
            if text[n - 1 - i] == pattern[1] {
                count1 += 1;
            }
        }

        ret0.max(ret1)
    }
}
```
