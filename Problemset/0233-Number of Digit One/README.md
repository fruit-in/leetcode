# 233. Number of Digit One
Given an integer `n`, count *the total number of digit* `1` *appearing in all non-negative integers less than or equal to* `n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 13
<strong>Output:</strong> 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 0
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>0 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let digits = n
            .to_string()
            .bytes()
            .map(|x| (x - b'0') as i32)
            .collect::<Vec<_>>();
        let mut tenpow_r = 10_i32.pow(digits.len() as u32 - 1);
        let mut part_l = 0;
        let mut part_r = n;
        let mut ret = 0;

        for &digit in &digits {
            part_r = n % tenpow_r;
            ret += part_l * tenpow_r
                + match digit {
                    0 => 0,
                    1 => part_r + 1,
                    _ => tenpow_r,
                };
            tenpow_r /= 10;
            part_l = part_l * 10 + digit;
        }

        ret
    }
}
```
