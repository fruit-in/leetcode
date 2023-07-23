# 1281. 整数的各位积和之差
给你一个整数 ```n```，请你帮忙计算并返回该整数「各位数字之积」与「各位数字之和」的差。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 234
<strong>输出:</strong> 15
<strong>解释:</strong>
各位数之积 = 2 * 3 * 4 = 24
各位数之和 = 2 + 3 + 4 = 9
结果 = 24 - 9 = 15
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4421
<strong>输出:</strong> 21
<strong>解释:</strong>
各位数之积 = 4 * 4 * 2 * 1 = 32
各位数之和 = 4 + 4 + 2 + 1 = 11
结果 = 32 - 11 = 21
</pre>

#### 提示:
* ```1 <= n <= 10^5```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::with_capacity(6);

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.iter().product::<i32>() - digits.iter().sum::<i32>()
    }
}
```
