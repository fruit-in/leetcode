# 2317. 操作后的最大异或和
给你一个下标从 **0** 开始的整数数组 `nums` 。一次操作中，选择 **任意** 非负整数 `x` 和一个下标 `i` ，**更新** `nums[i] 为 nums[i] AND (nums[i] XOR x)` 。

注意，`AND` 是逐位与运算，`XOR` 是逐位异或运算。

请你执行 **任意次** 更新操作，并返回 `nums` 中所有元素 **最大** 逐位异或和。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,2,4,6]
<strong>输出:</strong> 7
<strong>解释:</strong> 选择 x = 4 和 i = 3 进行操作，num[3] = 6 AND (6 XOR 4) = 6 AND 2 = 2 。
现在，nums = [3, 2, 4, 2] 且所有元素逐位异或得到 3 XOR 2 XOR 4 XOR 2 = 7 。
可知 7 是能得到的最大逐位异或和。
注意，其他操作可能也能得到逐位异或和 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,9,2]
<strong>输出:</strong> 11
<strong>解释:</strong> 执行 0 次操作。
所有元素的逐位异或和为 1 XOR 2 XOR 3 XOR 9 XOR 2 = 11 。
可知 11 是能得到的最大逐位异或和。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc | x)
    }
}
```
