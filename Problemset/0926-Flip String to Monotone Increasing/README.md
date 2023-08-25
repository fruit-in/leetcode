# 926. Flip String to Monotone Increasing
A binary string is monotone increasing if it consists of some number of `0`'s (possibly none), followed by some number of `1`'s (also possibly none).

You are given a binary string `s`. You can flip `s[i]` changing it from `0` to `1` or from `1` to `0`.

Return *the minimum number of flips to make* `s` *monotone increasing*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "00110"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We flip the last digit to get 00111.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "010110"
<strong>Output:</strong> 2
<strong>Explanation:</strong> We flip to get 011111, or alternatively 000111.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "00011000"
<strong>Output:</strong> 2
<strong>Explanation:</strong> We flip to get 00000000.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = s.iter().filter(|&&c| c == b'0').count() as i32;
        let mut ret = count;

        for i in 0..s.len() {
            count += (s[i] == b'1') as i32 - (s[i] == b'0') as i32;

            ret = ret.min(count);
        }

        ret
    }
}
```
