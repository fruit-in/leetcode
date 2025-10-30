# 2380. Time Needed to Rearrange a Binary String
You are given a binary string `s`. In one second, **all** occurrences of `"01"` are **simultaneously** replaced with `"10"`. This process **repeats** until no occurrences of `"01"` exist.

Return *the number of seconds needed to complete this process*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "0110101"
<strong>Output:</strong> 4
<strong>Explanation:</strong>
After one second, s becomes "1011010".
After another second, s becomes "1101100".
After the third second, s becomes "1110100".
After the fourth second, s becomes "1111000".
No occurrence of "01" exists any longer, and the process needed 4 seconds to complete,
so we return 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "11100"
<strong>Output:</strong> 0
<strong>Explanation:</strong>
No occurrence of "01" exists in s, and the processes needed 0 seconds to complete,
so we return 0.
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s[i]` is either `'0'` or `'1'`.

#### Follow up:
Can you solve this problem in O(n) time complexity?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut count0 = 0;
        let mut wait = 0;

        for c in s.trim_start_matches('1').trim_end_matches('0').chars() {
            if c == '0' {
                count0 += 1;
                wait -= 1;
            } else {
                wait = (wait + 1).max(0);
            }
        }

        count0 + wait
    }
}
```
