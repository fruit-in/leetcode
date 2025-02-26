# 523. 连续的子数组和
给你一个整数数组 `nums` 和一个整数 `k` ，如果 `nums` 有一个 **好的子数组** 返回 `true` ，否则返回 `false`：

一个 **好的子数组** 是：
* 长度 **至少为 2** ，且
* 子数组元素总和为 `k` 的倍数。

**注意**：
* **子数组** 是数组中 **连续** 的部分。
* 如果存在一个整数 `n` ，令整数 `x` 符合 `x = n * k` ，则称 `x` 是 `k` 的一个倍数。`0` **始终** 视为 `k` 的一个倍数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [23,2,4,6,7], k = 6
<strong>输出:</strong> true
<strong>解释:</strong> [2,4] 是一个大小为 2 的子数组，并且和为 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [23,2,6,4,7], k = 6
<strong>输出:</strong> true
<strong>解释:</strong> [23, 2, 6, 4, 7] 是大小为 5 的子数组，并且和为 42 。
42 是 6 的倍数，因为 42 = 7 * 6 且 7 是一个整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [23,2,6,4,7], k = 13
<strong>输出:</strong> false
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= sum(nums[i]) <= 2<sup>31</sup> - 1</code>
* <code>1 <= k <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_mod = 0;
        let mut mod_indices = HashMap::from([(0, -1)]);

        for i in 0..nums.len() {
            prefix_mod = (prefix_mod + nums[i]) % k;

            match mod_indices.get(&prefix_mod) {
                Some(j) if j + 1 < i as i32 => return true,
                None => {
                    mod_indices.insert(prefix_mod, i as i32);
                }
                _ => (),
            }
        }

        false
    }
}
```
