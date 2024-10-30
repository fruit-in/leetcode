# 1864. Minimum Number of Swaps to Make the Binary String Alternating
Given a binary string `s`, return *the **minimum** number of character swaps to make it **alternating**, or* `-1` *if it is impossible*.

The string is called **alternating** if no two adjacent characters are equal. For example, the strings `"010"` and `"1010"` are alternating, while the string `"0100"` is not.

Any two characters may be swapped, even if they are **not adjacent**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "111000"
<strong>Output:</strong> 1
<strong>Explanation:</strong> Swap positions 1 and 4: "111000" -> "101010"
The string is now alternating.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "010"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string is already alternating, no swaps are needed.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1110"
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;
        let mut ret0 = i32::MAX;
        let mut ret1 = i32::MAX;

        if zeros == ones || zeros == ones + 1 {
            ret0 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' != c)
                .count() as i32
                / 2;
        }
        if zeros == ones || zeros + 1 == ones {
            ret1 = s
                .bytes()
                .enumerate()
                .filter(|&(i, c)| (i % 2) as u8 + b'0' == c)
                .count() as i32
                / 2;
        }

        if ret0 == i32::MAX && ret1 == i32::MAX {
            return -1;
        }

        ret0.min(ret1)
    }
}
```
