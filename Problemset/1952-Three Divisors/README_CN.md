# 1952. 三除数
给你一个整数 `n` 。如果 `n` **恰好有三个正除数** ，返回 `true` ；否则，返回 `false` 。

如果存在整数 `k` ，满足 `n = k * m` ，那么整数 `m` 就是 `n` 的一个 **除数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> false
<strong>解释:</strong> 2 只有两个除数：1 和 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> true
<strong>解释:</strong> 4 有三个除数：1、2 和 4 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_three(n: i32) -> bool {
        (1..=n).filter(|m| n % m == 0).count() == 3
    }
}
```
