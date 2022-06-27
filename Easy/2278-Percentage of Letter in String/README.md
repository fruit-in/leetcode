# 2278. Percentage of Letter in String
Given a string `s` and a character `letter`, return *the **percentage** of characters in* `s` *that equal* `letter` ***rounded down** to the nearest whole percent*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "foobar", letter = "o"
<strong>Output:</strong> 33
<strong>Explanation:</strong>
The percentage of characters in s that equal the letter 'o' is 2 / 6 * 100% = 33% when rounded down, so we return 33.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "jjjj", letter = "k"
<strong>Output:</strong> 0
<strong>Explanation:</strong>
The percentage of characters in s that equal the letter 'k' is 0%, so we return 0.
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` consists of lowercase English letters.
* `letter` is a lowercase English letter.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|&c| c == letter).count() * 100 / s.len()) as i32
    }
}
```
