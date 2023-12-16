# 233. 数字 1 的个数
给定一个整数 `n`，计算所有小于等于 `n` 的非负整数中数字 `1` 出现的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 13
<strong>输出:</strong> 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 0
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>0 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
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
