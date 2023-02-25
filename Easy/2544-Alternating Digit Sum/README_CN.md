# 2544. 交替数字和
给你一个正整数 `n` 。`n` 中的每一位数字都会按下述规则分配一个符号：

* **最高有效位** 上的数字分配到 **正** 号。
* 剩余每位上数字的符号都与其相邻数字相反。

返回所有数字及其对应符号的和。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 521
<strong>输出:</strong> 4
<strong>解释:</strong> (+5) + (-2) + (+1) = 4.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 111
<strong>输出:</strong> 1
<strong>解释:</strong> (+1) + (-1) + (+1) = 1.
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 886996
<strong>输出:</strong> 0
<strong>解释:</strong> (+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0.
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n;
        let mut ret = 0;

        while n > 0 {
            ret = n % 10 - ret;
            n /= 10;
        }

        ret
    }
}
```
