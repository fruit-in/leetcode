# 1542. Find Longest Awesome Substring
You are given a string `s`. An **awesome** substring is a non-empty substring of `s` such that we can make any number of swaps in order to make it a palindrome.

Return *the length of the maximum length **awesome substring** of* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "3242415"
<strong>Output:</strong> 5
<strong>Explanation:</strong> "24241" is the longest awesome substring, we can form the palindrome "24142" with some swaps.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "12345678"
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "213123"
<strong>Output:</strong> 6
<strong>Explanation:</strong> "213123" is the longest awesome substring, we can form the palindrome "231132" with some swaps.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists only of digits.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut hash = HashMap::from([(0, -1)]);
        let mut x = 0;
        let mut ret = 1;

        for (i, digit) in s.bytes().enumerate() {
            x ^= 1 << (digit - b'0');
            ret = ret.max(i as i32 - hash.get(&x).unwrap_or(&(i as i32)));
            for digit in 0..10 {
                ret = ret.max(i as i32 - hash.get(&(x ^ (1 << digit))).unwrap_or(&(i as i32)));
            }
            if !hash.contains_key(&x) {
                hash.insert(x, i as i32);
            }
        }

        ret
    }
}
```
