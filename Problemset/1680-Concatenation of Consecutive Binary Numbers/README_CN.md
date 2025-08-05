# 1680. 连接连续二进制数字
给你一个整数 `n` ，请你将 `1` 到 `n` 的二进制表示连接起来，并返回连接结果对应的 **十进制** 数字对 <code>10<sup>9</sup> + 7</code> 取余的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 二进制的 "1" 对应着十进制的 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 27
<strong>解释:</strong> 二进制下，1，2 和 3 分别对应 "1" ，"10" 和 "11" 。
将它们依次连接，我们得到 "11011" ，对应着十进制的 27 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 12
<strong>输出:</strong> 505379714
<strong>解释:</strong> 连接结果为 "1101110010111011110001001101010111100" 。
对应的十进制数字为 118505380540 。
对 10<sup>9</sup> + 7 取余后，结果为 505379714 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        fn pow_of_2(exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) % 1_000_000_007
            } else {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) * 2 % 1_000_000_007
            }
        }

        let mut shl = 0;
        let mut ret = 0;

        for x in (1..=n as i64).rev() {
            ret = (ret + x * pow_of_2(shl)) % 1_000_000_007;
            shl += x.ilog2() + 1;
        }

        ret as i32
    }
}
```
