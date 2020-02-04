# 400. 第N个数字
在无限的整数序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...中找到第 *n* 个数字。

#### 注意:
*n* 是正数且在32为整形范围内 ( *n* < 2<sup>31</sup>)。

#### 示例 1:
<pre>
<strong>输入:</strong>
3
<strong>输出:</strong>
3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
11
<strong>输出:</strong>
0
<strong>说明:</strong>
第11个数字在序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... 里是<strong>0</strong>，它是10的一部分。
</pre>

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n;
        let mut a = 0;
        let mut b = 0;
        let mut c = 9;

        for i in 0..8 {
            a += 9 * (i + 1) * 10_i32.pow(i as u32);
            if a >= n {
                c = i + 1;
                break;
            }
            b = a;
        }

        n -= b + 1;
        match n % c {
            0 => 1 + n / (c * 10_i32.pow(c as u32 - 1)),
            d => n % (c * 10_i32.pow(c as u32 - d as u32)) / (c * 10_i32.pow(c as u32 - d as u32 - 1)),
        }
    }
}
```
