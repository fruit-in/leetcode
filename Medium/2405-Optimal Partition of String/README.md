# 2405. Optimal Partition of String
Given a string `s`, partition the string into one or more **substrings** such that the characters in each substring are **unique**. That is, no letter appears in a single substring more than **once**.

Return *the **minimum** number of substrings in such a partition*.

Note that each character should belong to exactly one substring in a partition.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abacaba"
<strong>Output:</strong> 4
<strong>Explanation:</strong>
Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
It can be shown that 4 is the minimum number of substrings needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ssssss"
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The only valid partition is ("s","s","s","s","s","s").
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only English lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut mask = 0;
        let mut ret = 1;

        for c in s.bytes() {
            if (1 << (c - b'a')) & mask != 0 {
                mask = 0;
                ret += 1;
            }

            mask |= 1 << (c - b'a');
        }

        ret
    }
}
```
