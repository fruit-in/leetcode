# 2413. 最小偶倍数
给你一个正整数 `n` ，返回 `2` 和 `n` 的最小公倍数（正整数）。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5
<strong>输出:</strong> 10
<strong>解释:</strong> 5 和 2 的最小公倍数是 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6
<strong>输出:</strong> 6
<strong>解释:</strong> 6 和 2 的最小公倍数是 6 。注意数字会是它自身的倍数。
</pre>

#### 提示:
* `1 <= n <= 150`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            0 => n,
            _ => 2 * n,
        }
    }
}
```
