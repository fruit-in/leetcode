# 2429. 最小 XOR
给你两个正整数 `num1` 和 `num2` ，找出满足下述条件的整数 `x` ：

* `x` 的置位数和 `num2` 相同，且
* `x XOR num1` 的值 **最小**

注意 `XOR` 是按位异或运算。

返回整数 `x` 。题目保证，对于生成的测试用例， `x` 是 **唯一确定** 的。

整数的 **置位数** 是其二进制表示中 `1` 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> num1 = 3, num2 = 5
<strong>输出:</strong> 3
<strong>解释:</strong>
num1 和 num2 的二进制表示分别是 0011 和 0101 。
整数 3 的置位数与 num2 相同，且 3 XOR 3 = 0 是最小的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num1 = 1, num2 = 12
<strong>输出:</strong> 3
<strong>解释:</strong>
num1 和 num2 的二进制表示分别是 0001 和 1100 。
整数 3 的置位数与 num2 相同，且 3 XOR 1 = 2 是最小的。
</pre>

#### 提示:
* <code>1 <= num1, num2 <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let ones1 = num1.count_ones() as i32;
        let ones2 = num2.count_ones() as i32;
        let mut i = 0;
        let mut x = num1;

        for _ in 0..(ones1 - ones2).max(ones2 - ones1) {
            if ones1 > ones2 {
                while x & (1 << i) == 0 {
                    i += 1;
                }

                x ^= 1 << i;
            } else {
                while x & (1 << i) != 0 {
                    i += 1;
                }

                x |= 1 << i;
            }
        }

        x
    }
}
```
