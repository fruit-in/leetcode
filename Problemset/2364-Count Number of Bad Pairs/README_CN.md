# 2364. 统计坏数对的数目
给你一个下标从 **0** 开始的整数数组 `nums` 。如果 `i < j` 且 `j - i != nums[j] - nums[i]` ，那么我们称 `(i, j)` 是一个 **坏数对** 。

请你返回 `nums` 中 **坏数对** 的总数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,1,3,3]
<strong>输出:</strong> 5
<strong>输出:</strong> 数对 (0, 1) 是坏数对，因为 1 - 0 != 1 - 4 。
数对 (0, 2) 是坏数对，因为 2 - 0 != 3 - 4, 2 != -1 。
数对 (0, 3) 是坏数对，因为 3 - 0 != 3 - 4, 3 != -1 。
数对 (1, 2) 是坏数对，因为 2 - 1 != 3 - 1, 1 != 2 。
数对 (2, 3) 是坏数对，因为 3 - 2 != 3 - 3, 1 != 0 。
总共有 5 个坏数对，所以我们返回 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> 0
<strong>输出:</strong> 没有坏数对。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            ret += i as i64 - count.get(&(i as i32 - nums[i])).unwrap_or(&0);
            count
                .entry(i as i32 - nums[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        ret
    }
}
```
