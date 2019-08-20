# 50. Pow(x, n)
实现<code>pow(*x*, *n*)</code>，即计算 x 的 n 次幂函数。

#### 示例 1:
<pre>
<strong>输入:</strong> 2.00000, 10
<strong>输出:</strong> 1024.00000
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 2.10000, 3
<strong>输出:</strong> 9.26100
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 2.00000, -2
<strong>输出:</strong> 0.25000
<strong>解释:</strong> 2<sup>-2</sup> = 1/2<sup>2</sup> = 1/4 = 0.25
</pre>

#### 说明:
* -100.0 < *x* < 100.0
* *n* 是 32 位有符号整数，其数值范围是 [−2<sup>31</sup>, 2<sup>31</sup> − 1] 。

## 题解 (Rust)

### 1. 递归
```Rust
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n > 0 {
            let x_n2 = Self::my_pow(x, n / 2);
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 * x
            }
        } else if n < 0 {
            let x_n2 = Self::my_pow(1.0 / x, -(n / 2));
            if n % 2 == 0 {
                x_n2 * x_n2
            } else {
                x_n2 * x_n2 / x
            }
        } else {
            1.0
        }
    }
}
```
