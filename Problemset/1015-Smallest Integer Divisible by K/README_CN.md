# 1015. 可被 K 整除的最小整数
给定正整数 `k` ，你需要找出可以被 `k` 整除的、仅包含数字 `1` 的最 **小** 正整数 `n` 的长度。

返回 `n` 的长度。如果不存在这样的 `n` ，就返回-1。

注意： `n` 可能不符合 64 位带符号整数。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 最小的答案是 n = 1，其长度为 1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 不存在可被 2 整除的正整数 n 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 最小的答案是 n = 111，其长度为 3。
</pre>

#### 提示:
* <code>1 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut rem = 0;

        for n in 1..=k {
            rem = (rem * 10 % k + 1 % k) % k;

            if rem == 0 {
                return n;
            }
        }

        -1
    }
}
```
