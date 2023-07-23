# 2390. Removing Stars From a String
You are given a string `s`, which contains stars `*`.

In one operation, you can:

* Choose a star in `s`.
* Remove the closest **non-star** character to its **left**, as well as remove the star itself.

Return *the string after **all** stars have been removed*.

#### Note:
* The input will be generated such that the operation is always possible.
* It can be shown that the resulting string will always be unique.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leet**cod*e"
<strong>Output:</strong> "lecoe"
<strong>Explanation:</strong> Performing the removals from left to right:
- The closest character to the 1st star is 't' in "leet**cod*e". s becomes "lee*cod*e".
- The closest character to the 2nd star is 'e' in "lee*cod*e". s becomes "lecod*e".
- The closest character to the 3rd star is 'd' in "lecod*e". s becomes "lecoe".
There are no more stars, so we return "lecoe".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "erase*****"
<strong>Output:</strong> ""
<strong>Explanation:</strong> The entire string is removed, so we return an empty string.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters and stars `*`.
* The operation above can be performed on `s`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut ret = vec![];

        for b in s.bytes() {
            if b == b'*' {
                ret.pop();
            } else {
                ret.push(b);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
