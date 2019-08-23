# 58. Length of Last Word
Given a string *s* consists of upper/lower-case alphabets and empty space characters ```' '```, return the length of last word in the string.

If the last word does not exist, return 0.

**Note:** A word is defined as a character sequence consists of non-space characters only.

#### Example:
<pre>
<strong>Input:</strong> "Hello World"
<strong>Output:</strong> 5
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut cnt = 0;
        let mut flag = false;
        for c in s.chars() {
            if c == ' ' {
                flag = true;
            } else if flag {
                cnt = 1;
                flag = false;
            } else {
                cnt += 1;
            }
        }
        cnt
    }
}
```
