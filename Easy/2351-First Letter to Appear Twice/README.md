# 2351. First Letter to Appear Twice
Given a string `s` consisting of lowercase English letters, return *the first letter to appear **twice***.

#### Note:
* A letter `a` appears twice before another letter `b` if the **second** occurrence of `a` is before the **second** occurrence of `b`.
* `s` will contain at least one letter that appears twice.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abccbaacz"
<strong>Output:</strong> "c"
<strong>Explanation:</strong>
The letter 'a' appears on the indexes 0, 5 and 6.
The letter 'b' appears on the indexes 1 and 4.
The letter 'c' appears on the indexes 2, 3 and 7.
The letter 'z' appears on the index 8.
The letter 'c' is the first letter to appear twice, because out of all the letters the index of its second occurrence is the smallest.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcdd"
<strong>Output:</strong> "d"
<strong>Explanation:</strong>
The only letter that appears twice is 'd' so we return 'd'.
</pre>

#### Constraints:
* `2 <= s.length <= 100`
* `s` consists of lowercase English letters.
* `s` has at least one repeated letter.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut mask = 0;

        for c in s.chars() {
            if mask & (1 << (c as i32 - 97)) != 0 {
                return c;
            }

            mask |= 1 << (c as i32 - 97);
        }

        unimplemented!()
    }
}
```
