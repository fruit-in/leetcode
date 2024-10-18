# 1969. 数组元素的最小非零乘积
给你一个正整数 `p` 。你有一个下标从 **1** 开始的数组 `nums` ，这个数组包含范围 <code>[1, 2<sup>p</sup> - 1]</code> 内所有整数的二进制形式（两端都 **包含**）。你可以进行以下操作 **任意** 次：

* 从 `nums` 中选择两个元素 `x` 和 `y`  。
* 选择 `x` 中的一位与 `y` 对应位置的位交换。对应位置指的是两个整数 **相同位置** 的二进制位。

比方说，如果 `x = 1101` 且 `y = 0011` ，交换右边数起第 `2` 位后，我们得到 `x = 1111` 和 `y = 0001` 。

请你算出进行以上操作 **任意次** 以后，`nums` 能得到的 **最小非零** 乘积。将乘积对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

**注意：**答案应为取余 **之前** 的最小值。

#### 示例 1:
<pre>
<strong>输入:</strong> p = 1
<strong>输出:</strong> 1
<strong>解释:</strong> nums = [1] 。
只有一个元素，所以乘积为该元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> p = 2
<strong>输出:</strong> 6
<strong>解释:</strong> nums = [01, 10, 11] 。
所有交换要么使乘积变为 0 ，要么乘积与初始乘积相同。
所以，数组乘积 1 * 2 * 3 = 6 已经是最小值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> p = 3
<strong>输出:</strong> 1512
<strong>解释:</strong> nums = [001, 010, 011, 100, 101, 110, 111]
- 第一次操作中，我们交换第二个和第五个元素最左边的数位。
    - 结果数组为 [001, 110, 011, 100, 001, 110, 111] 。
- 第二次操作中，我们交换第三个和第四个元素中间的数位。
    - 结果数组为 [001, 110, 001, 110, 001, 110, 111] 。
数组乘积 1 * 6 * 1 * 6 * 1 * 6 * 7 = 1512 是最小乘积。
</pre>

#### 提示:
* `1 <= p <= 60`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let x = 2_u64.pow(p as u32) % 1_000_000_007;

        ((x - 1) * Self::power(x - 2, 2_u64.pow(p as u32 - 1) - 1) % 1_000_000_007) as i32
    }

    fn power(x: u64, exp: u64) -> u64 {
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let y = Self::power(x, exp / 2);
            y * y % 1_000_000_007
        } else {
            x * Self::power(x, exp - 1) % 1_000_000_007
        }
    }
}
```