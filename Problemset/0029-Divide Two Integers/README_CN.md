# 29. 两数相除
给你两个整数，被除数 `dividend` 和除数 `divisor`。将两数相除，要求 **不使用** 乘法、除法和取余运算。

整数除法应该向零截断，也就是截去（`truncate`）其小数部分。例如，`8.345` 将被截断为 `8` ，`-2.7335` 将被截断至 `-2` 。

返回被除数 `dividend` 除以除数 `divisor` 得到的 **商** 。

**注意：**假设我们的环境只能存储 **32 位** 有符号整数，其数值范围是 <code>[−2<sup>31</sup>,  2<sup>31</sup> − 1]</code> 。本题中，如果商 **严格大于** <code>2<sup>31</sup> − 1</code> ，则返回 <code>2<sup>31</sup> − 1</code> ；如果商 **严格小于** <code>-2<sup>31</sup></code> ，则返回 <code>-2<sup>31</sup></code> 。

#### 示例 1:
<pre>
<strong>输入:</strong> dividend = 10, divisor = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 10/3 = 3.33333.. ，向零截断后得到 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> dividend = 7, divisor = -3
<strong>输出:</strong> -2
<strong>解释:</strong> 7/-3 = -2.33333.. ，向零截断后得到 -2 。
</pre>

#### 提示:
* <code>-2<sup>31</sup> <= dividend, divisor <= 2<sup>31</sup> - 1</code>
* `divisor != 0`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let is_neg = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut ret = 0;

        while dividend >= divisor {
            for i in 1..=32 {
                if divisor << i > dividend {
                    dividend -= divisor << (i - 1);
                    ret += 1 << (i - 1);
                    break;
                }
            }
        }

        if is_neg {
            ret = -ret;
        }

        ret
    }
}
```
