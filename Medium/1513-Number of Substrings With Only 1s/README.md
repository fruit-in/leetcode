# 1513. Number of Substrings With Only 1s
Given a binary string `s` (a string consisting only of '0' and '1's).

Return the number of substrings with all characters 1's.

Since the answer may be too large, return it modulo 10^9 + 7.

#### Example 1:
<pre>
<b>Input:</b> s = "0110111"
<b>Output:</b> 9
<b>Explanation:</b> There are 9 substring in total with only 1's characters.
"1" -> 5 times.
"11" -> 3 times.
"111" -> 1 time.
</pre>

#### Example 2:
<pre>
<b>Input:</b> s = "101"
<b>Output:</b> 2
<b>Explanation:</b> Substring "1" is shown 2 times in s.
</pre>

#### Example 3:
<pre>
<b>Input:</b> s = "111111"
<b>Output:</b> 21
<b>Explanation:</b> Each substring contains only 1's characters.
</pre>

#### Example 4:
<pre>
<b>Input:</b> s = "000"
<b>Output:</b> 0
</pre>

#### Constraints:
* `s[i] == '0'` or `s[i] == '1'`
* `1 <= s.length <= 10^5`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        (s.split('0')
            .map(|ones| ones.len())
            .fold(0, |acc, x| acc + (x + 1) * x / 2)
            % 1000000007) as i32
    }
}
```
