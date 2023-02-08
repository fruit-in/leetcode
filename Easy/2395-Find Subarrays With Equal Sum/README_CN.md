# 2395. 和相等的子数组
给你一个下标从 **0** 开始的整数数组 `nums` ，判断是否存在 **两个** 长度为 `2` 的子数组且它们的 **和** 相等。注意，这两个子数组起始位置的下标必须 **不相同** 。

如果这样的子数组存在，请返回 `true`，否则返回 `false` 。

**子数组** 是一个数组中一段连续非空的元素组成的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,2,4]
<strong>输出:</strong> true
<strong>解释:</strong> 元素为 [4,2] 和 [2,4] 的子数组有相同的和 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> false
<strong>解释:</strong> 没有长度为 2 的两个子数组和相等。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0,0,0]
<strong>输出:</strong> true
<strong>解释:</strong> 子数组 [nums[0],nums[1]] 和 [nums[1],nums[2]] 的和相等，都为 0 。
注意即使子数组的元素相同，这两个子数组也视为不相同的子数组，因为它们在原数组中的起始位置不同。
</pre>

#### 提示:
* `2 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut subarray_sums = HashSet::new();

        for i in 1..nums.len() {
            if subarray_sums.contains(&(nums[i] + nums[i - 1])) {
                return true;
            }
            subarray_sums.insert(nums[i] + nums[i - 1]);
        }

        false
    }
}
```
