# 2177. 找到和为给定整数的三个连续整数
给你一个整数 `num` ，请你返回三个连续的整数，它们的 **和** 为 `num` 。如果 `num` 无法被表示成三个连续整数的和，请你返回一个 **空** 数组。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 33
<strong>输出:</strong> [10,11,12]
<strong>解释:</strong> 33 可以表示为 10 + 11 + 12 = 33 。
10, 11, 12 是 3 个连续整数，所以返回 [10, 11, 12] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 4
<strong>输出:</strong> []
<strong>解释:</strong> 没有办法将 4 表示成 3 个连续整数的和。
</pre>

#### 提示:
* <code>0 <= num <= 10<sup>15</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 == 0 {
            vec![num / 3 - 1, num / 3, num / 3 + 1]
        } else {
            vec![]
        }
    }
}
```
