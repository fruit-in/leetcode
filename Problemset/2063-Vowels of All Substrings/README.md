# 2063. Vowels of All Substrings
Given a string `word`, return *the **sum of the number of vowels*** (`'a'`, `'e'`, `'i'`, `'o'`, *and* `'u'`) *in every substring of* `word`.

A **substring** is a contiguous (non-empty) sequence of characters within a string.

**Note:** Due to the large constraints, the answer may not fit in a signed 32-bit integer. Please be careful during the calculations.

#### Example 1:
<pre>
<strong>Input:</strong> word = "aba"
<strong>Output:</strong> 6
<strong>Explanation:</strong>
All possible substrings are: "a", "ab", "aba", "b", "ba", and "a".
- "b" has 0 vowels in it
- "a", "ab", "ba", and "a" have 1 vowel each
- "aba" has 2 vowels in it
Hence, the total sum of vowels = 0 + 1 + 1 + 1 + 1 + 2 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "abc"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
All possible substrings are: "a", "ab", "abc", "b", "bc", and "c".
- "a", "ab", and "abc" have 1 vowel each
- "b", "bc", and "c" have 0 vowels each
Hence, the total sum of vowels = 1 + 1 + 1 + 0 + 0 + 0 = 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "ltcd"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no vowels in any substring of "ltcd".
</pre>

#### Constraints:
* <code>1 <= word.length <= 10<sup>5</sup></code>
* `word` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let word = word.as_bytes();
        let mut x = 0;
        let mut ret = 0;

        for i in 0..word.len() {
            if [b'a', b'e', b'i', b'o', b'u'].contains(&word[i]) {
                x += i as i64 + 1;
            }

            ret += x;
        }

        ret
    }
}
```
