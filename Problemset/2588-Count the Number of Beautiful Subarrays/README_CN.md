# 2588. 统计美丽子数组数目
给你一个下标从 **0** 开始的整数数组`nums` 。每次操作中，你可以：
* 选择两个满足 `0 <= i, j < nums.length` 的不同下标 `i` 和 `j` 。
* 选择一个非负整数 `k` ，满足 `nums[i]` 和 `nums[j]` 在二进制下的第 `k` 位（下标编号从 0 开始）是 `1` 。
* 将 `nums[i]` 和 `nums[j]` 都减去 <code>2<sup>k</sup></code> 。

如果一个子数组内执行上述操作若干次（包括 0 次）后，该子数组可以变成一个全为 `0` 的数组，那么我们称它是一个 **美丽** 的子数组。

请你返回数组 `nums` 中 **美丽子数组** 的数目。

子数组是一个数组中一段连续 **非空** 的元素序列。

**注意：**所有元素最初都是 0 的子数组被认为是美丽的，因为不需要进行任何操作。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,3,1,2,4]
<strong>输出:</strong> 2
<strong>解释:</strong> nums 中有 2 个美丽子数组：[4,3,1,2,4] 和 [4,3,1,2,4] 。
- 按照下述步骤，我们可以将子数组 [3,1,2] 中所有元素变成 0 ：
  - 选择 [3, 1, 2] 和 k = 1 。将 2 个数字都减去 21 ，子数组变成 [1, 1, 0] 。
  - 选择 [1, 1, 0] 和 k = 0 。将 2 个数字都减去 20 ，子数组变成 [0, 0, 0] 。
- 按照下述步骤，我们可以将子数组 [4,3,1,2,4] 中所有元素变成 0 ：
  - 选择 [4, 3, 1, 2, 4] 和 k = 2 。将 2 个数字都减去 22 ，子数组变成 [0, 3, 1, 2, 0] 。
  - 选择 [0, 3, 1, 2, 0] 和 k = 0 。将 2 个数字都减去 20 ，子数组变成 [0, 2, 0, 2, 0] 。
  - 选择 [0, 2, 0, 2, 0] 和 k = 1 。将 2 个数字都减去 21 ，子数组变成 [0, 0, 0, 0, 0] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,10,4]
<strong>输出:</strong> 0
<strong>解释:</strong> nums 中没有任何美丽子数组。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut mask = 0;
        let mut mask_count = HashMap::from([(0, 1)]);
        let mut ret = 0;

        for &x in &nums {
            mask ^= x;
            ret += mask_count.get(&mask).unwrap_or(&0);
            *mask_count.entry(mask).or_insert(0) += 1;
        }

        ret
    }
}
```
