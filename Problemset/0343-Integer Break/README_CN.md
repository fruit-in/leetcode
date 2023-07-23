# 343. 整数拆分
给定一个正整数 *n*，将其拆分为**至少**两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 1
<strong>解释:</strong> 2 = 1 + 1, 1 × 1 = 1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 10
<strong>输出:</strong> 36
<strong>解释:</strong> 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
</pre>

**说明:** 你可以假设 *n* 不小于 2 且不大于 58。

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match (n, (n - 2) / 3) {
            (2, _) | (3, _) => n - 1,
            (_, x) => 3i32.pow(x as u32) * (n - 3 * x),
        }
    }
}
```
