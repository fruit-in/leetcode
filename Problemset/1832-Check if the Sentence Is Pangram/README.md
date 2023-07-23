# 1832. Check if the Sentence Is Pangram
A **pangram** is a sentence where every letter of the English alphabet appears at least once.

Given a string `sentence` containing only lowercase English letters, return `true` *if* `sentence` *is a **pangram**, or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> sentence = "thequickbrownfoxjumpsoverthelazydog"
<strong>Output:</strong> true
<strong>Explanation:</strong> sentence contains at least one of every letter of the English alphabet.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> sentence = "leetcode"
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= sentence.length <= 1000`
* `sentence` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence.bytes().fold(0, |acc, c| match c {
            b'A'..=b'Z' => acc | (1 << (c - b'A')),
            b'a'..=b'z' => acc | (1 << (c - b'a')),
            _ => acc,
        }) == (1 << 26) - 1
    }
}
```
