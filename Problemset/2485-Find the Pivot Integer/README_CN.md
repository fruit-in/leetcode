# 2485. 找出中枢整数
给你一个正整数 `n` ，找出满足下述条件的 **中枢整数** `x` ：

* `1` 和 `x` 之间的所有元素之和等于 `x` 和 `n` 之间所有元素之和。

返回中枢整数 `x` 。如果不存在中枢整数，则返回 `-1` 。题目保证对于给定的输入，至多存在一个中枢整数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 8
<strong>输出:</strong> 6
<strong>解释:</strong> 6 是中枢整数，因为 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 1 是中枢整数，因为 1 = 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> -1
<strong>解释:</strong> 可以证明不存在满足题目要求的整数。
</pre>

#### 提示:
* `1 <= n <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let x = ((n * (n + 1) / 2) as f64).sqrt() as i32;

        if x * x * 2 == n * (n + 1) {
            x
        } else {
            -1
        }
    }
}
```
