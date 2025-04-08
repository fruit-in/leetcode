# 2461. 长度为 K 子数组中的最大和
给你一个整数数组 `nums` 和一个整数 `k` 。请你从 `nums` 中满足下述条件的全部子数组中找出最大子数组和：
* 子数组的长度是 `k`，且
* 子数组中的所有元素 **各不相同** 。

返回满足题面要求的最大子数组和。如果不存在子数组满足这些条件，返回 `0` 。

**子数组** 是数组中一段连续非空的元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,5,4,2,9,9,9], k = 3
<strong>输出:</strong> 15
<strong>解释:</strong> nums 中长度为 3 的子数组是：
- [1,5,4] 满足全部条件，和为 10 。
- [5,4,2] 满足全部条件，和为 11 。
- [4,2,9] 满足全部条件，和为 15 。
- [2,9,9] 不满足全部条件，因为元素 9 出现重复。
- [9,9,9] 不满足全部条件，因为元素 9 出现重复。
因为 15 是满足全部条件的所有子数组中的最大子数组和，所以返回 15 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,4,4], k = 3
<strong>输出:</strong> 0
<strong>解释:</strong> nums 中长度为 3 的子数组是：
- [4,4,4] 不满足全部条件，因为元素 4 出现重复。
因为不存在满足全部条件的子数组，所以返回 0 。
</pre>

#### 提示:
* <code>1 <= k <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut subarray_sum = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..k {
            subarray_sum += nums[j] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
        }
        if count.len() == k {
            ret = subarray_sum;
        }

        for j in k..nums.len() {
            subarray_sum += nums[j] as i64;
            subarray_sum -= nums[i] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
            *count.get_mut(&nums[i]).unwrap() -= 1;
            if count[&nums[i]] == 0 {
                count.remove(&nums[i]);
            }
            if count.len() == k {
                ret = ret.max(subarray_sum);
            }
            i += 1;
        }

        ret
    }
}
```
