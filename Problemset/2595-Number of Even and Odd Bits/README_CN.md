# 2595. 奇偶位数
给你一个 **正** 整数 `n` 。

用 `even` 表示在 `n` 的二进制形式（下标从 **0** 开始）中值为 `1` 的偶数下标的个数。

用 `odd` 表示在 `n` 的二进制形式（下标从 **0** 开始）中值为 `1` 的奇数下标的个数。

返回整数数组 `answer` ，其中 `answer = [even, odd]` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 17
<strong>输出:</strong> [2,0]
<strong>解释:</strong> 17 的二进制形式是 10001 。
下标 0 和 下标 4 对应的值为 1 。
共有 2 个偶数下标，0 个奇数下标。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> [0,1]
<strong>解释:</strong> 2 的二进制形式是 10 。
下标 1 对应的值为 1 。
共有 0 个偶数下标，1 个奇数下标。
</pre>

#### 提示:
* `1 <= n <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        vec![
            (n & 0x555).count_ones() as i32,
            (n & 0xaaa).count_ones() as i32,
        ]
    }
}
```
