# 520. Detect Capital
Given a word, you need to judge whether the usage of capitals in it is right or not.

We define the usage of capitals in a word to be right when one of the following cases holds:
1. All letters in this word are capitals, like "USA".
2. All letters in this word are not capitals, like "leetcode".
3. Only the first letter in this word is capital, like "Google".

Otherwise, we define that this word doesn't use capitals in a right way.

#### Example 1:
<pre>
<strong>Input:</strong> "USA"
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "FlaG"
<strong>Output:</strong> False
</pre>

**Note:** The input will be a non-empty word consisting of uppercase and lowercase latin letters.

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        if let Some(ch) = chars.next() {
            if ch.is_ascii_uppercase() {
                if let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_lowercase() {
                                return false;
                            }
                        }
                    } else {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_uppercase() {
                                return false;
                            }
                        }
                    }
                }
            } else {
                while let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
```
