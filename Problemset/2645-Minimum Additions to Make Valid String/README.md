# 2645. Minimum Additions to Make Valid String
Given a string `word` to which you can insert letters "a", "b" or "c" anywhere and any number of times, return *the minimum number of letters that must be inserted so that `word` becomes **valid***.

A string is called **valid** if it can be formed by concatenating the string "abc" several times.

#### Example 1:
<pre>
<strong>Input:</strong> word = "b"
<strong>Output:</strong> 2
<strong>Explanation:</strong> Insert the letter "a" right before "b", and the letter "c" right next to "b" to obtain the valid string "abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "aaa"
<strong>Output:</strong> 6
<strong>Explanation:</strong> Insert letters "b" and "c" next to each "a" to obtain the valid string "abcabcabc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "abc"
<strong>Output:</strong> 0
<strong>Explanation:</strong> word is already valid. No modifications are needed.
</pre>

#### Constraints:
* `1 <= word.length <= 50`
* `word` consists of letters "a", "b" and "c" only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut want = b'a';
        let mut ret = 0;

        for ch in word.bytes() {
            ret += (ch + 3 - want) as i32 % 3;
            want = (ch - b'a' + 1) % 3 + b'a';
        }

        ret += 2 - (want - b'a' + 2) as i32 % 3;

        ret
    }
}
```
