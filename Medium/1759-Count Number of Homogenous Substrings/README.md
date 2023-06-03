# 1759. Count Number of Homogenous Substrings
Given a string `s`, return *the number of **homogenous** substrings of* `s`. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A string is **homogenous** if all the characters of the string are the same.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abbcccaa"
<strong>Output:</strong> 13
<strong>Explanation:</strong> The homogenous substrings are listed as below:
"a"   appears 3 times.
"aa"  appears 1 time.
"b"   appears 2 times.
"bb"  appears 1 time.
"c"   appears 3 times.
"cc"  appears 2 times.
"ccc" appears 1 time.
3 + 1 + 2 + 1 + 3 + 2 + 1 = 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "xy"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The homogenous substrings are "x" and "y".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "zzzzz"
<strong>Output:</strong> 15
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                count = 0;
            }

            count += 1;
            ret = (ret + count) % 1_000_000_007;
        }

        ret
    }
}
```
