# 1941. Check if All Characters Have Equal Number of Occurrences
Given a string `s`, return `true` *if* `s` *is a **good** string, or* `false` *otherwise*.

A string `s` is **good** if **all** the characters that appear in `s` have the **same** number of occurrences (i.e., the same frequency).

#### Example 1:
<pre>
<strong>Input:</strong> s = "abacbc"
<strong>Output:</strong> true
<strong>Explanation:</strong> The characters that appear in s are 'a', 'b', and 'c'. All characters occur 2 times in s.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaabb"
<strong>Output:</strong> false
<strong>Explanation:</strong> The characters that appear in s are 'a' and 'b'.
'a' occurs 3 times while 'b' occurs 2 times, which is not the same number of times.
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut count = [0; 26];

        s.bytes().for_each(|c| count[(c - b'a') as usize] += 1);

        for i in 1..26 {
            if count[i] == 0 {
                count[i] = count[i - 1];
            } else if count[i - 1] != 0 && count[i] != count[i - 1] {
                return false;
            }
        }

        true
    }
}
```
