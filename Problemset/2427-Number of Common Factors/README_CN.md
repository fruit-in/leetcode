# 2427. 公因子的数目
给你两个正整数 `a` 和 `b` ，返回 `a` 和 `b` 的 **公** 因子的数目。

如果 `x` 可以同时整除 `a` 和 `b` ，则认为 `x` 是 `a` 和 `b` 的一个 **公因子** 。

#### 示例 1:
<pre>
<strong>输入:</strong> a = 12, b = 6
<strong>输出:</strong> 4
<strong>解释:</strong> 12 和 6 的公因子是 1、2、3、6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = 25, b = 30
<strong>输出:</strong> 2
<strong>解释:</strong> 25 和 30 的公因子是 1、5 。
</pre>

#### 提示:
* `1 <= a, b <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).filter(|x| a % x == 0 && b % x == 0).count() as i32
    }
}
```
