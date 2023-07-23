# 2160. 拆分数位后四位数字的最小和
给你一个四位 **正** 整数 `num` 。请你使用 `num` 中的 **数位** ，将 `num` 拆成两个新的整数 `new1` 和 `new2` 。`new1` 和 `new2` 中可以有 **前导 0** ，且 `num` 中 **所有** 数位都必须使用。
* 比方说，给你 `num = 2932` ，你拥有的数位包括：两个 `2` ，一个 `9` 和一个 `3` 。一些可能的 `[new1, new2]` 数对为 `[22, 93]`，`[23, 92]`，`[223, 9]` 和 `[2, 329]` 。

请你返回可以得到的 `new1` 和 `new2` 的 **最小** 和。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 2932
<strong>输出:</strong> 52
<strong>解释:</strong> 可行的 [new1, new2] 数对为 [29, 23] ，[223, 9] 等等。
最小和为数对 [29, 23] 的和：29 + 23 = 52 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 4009
<strong>输出:</strong> 13
<strong>解释:</strong> 可行的 [new1, new2] 数对为 [0, 49] ，[490, 0] 等等。
最小和为数对 [4, 9] 的和：4 + 9 = 13 。
</pre>

#### 提示:
* `1000 <= num <= 9999`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits = [num / 1000, num / 100 % 10, num / 10 % 10, num % 10];
        digits.sort_unstable();

        digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
    }
}
```
