# 2552. 统计上升四元组
给你一个长度为 `n` 下标从 0 开始的整数数组 `nums` ，它包含 `1` 到 `n` 的所有数字，请你返回上升四元组的数目。

如果一个四元组 `(i, j, k, l)` 满足以下条件，我们称它是上升的：
* `0 <= i < j < k < l < n` 且
* `nums[i] < nums[k] < nums[j] < nums[l]` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,2,4,5]
<strong>输出:</strong> 2
<strong>解释:</strong>
- 当 i = 0 ，j = 1 ，k = 2 且 l = 3 时，有 nums[i] < nums[k] < nums[j] < nums[l] 。
- 当 i = 0 ，j = 1 ，k = 2 且 l = 4 时，有 nums[i] < nums[k] < nums[j] < nums[l] 。
没有其他的四元组，所以我们返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 0
<strong>解释:</strong> 只存在一个四元组 i = 0 ，j = 1 ，k = 2 ，l = 3 ，但是 nums[j] < nums[k] ，所以我们返回 0 。
</pre>

#### 提示:
* `4 <= nums.length <= 4000`
* `1 <= nums[i] <= nums.length`
* `nums` 中所有数字 **互不相同** ，`nums` 是一个排列。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let mut n = nums.len();
        let mut counti = HashMap::new();
        let mut ret = 0;

        for k in 2..n - 1 {
            let mut count = 0;

            for j in 0..k {
                if nums[j] > nums[k] && count > 0 {
                    counti.insert((j, k), count);
                } else if nums[j] < nums[k] {
                    count += 1;
                }
            }
        }

        for j in 1..n - 2 {
            let mut count = 0;

            for k in (j + 1..n).rev() {
                if nums[k] < nums[j] && count > 0 {
                    ret += count * counti.get(&(j, k)).unwrap_or(&0);
                } else if nums[k] > nums[j] {
                    count += 1;
                }
            }
        }

        ret
    }
}
```
