# 172. 阶乘后的零
给定一个整数 *n*，返回 *n*! 结果尾数中零的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 0
<strong>解释:</strong> 3! = 6, 尾数中没有零。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 5
<strong>输出:</strong> 1
<strong>解释:</strong> 5! = 120, 尾数中有 1 个零.
</pre>

**说明:** 你算法的时间复杂度应为 *O*(log *n*) 。

## 题解 (Rust)

### 1. 计数5
```Rust
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut zeroes = 0;
        while n >= 5 {
            n /= 5;
            zeroes += n;
        }
        zeroes
    }
}
```
