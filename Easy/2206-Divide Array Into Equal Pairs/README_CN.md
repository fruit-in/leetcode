# 2206. 将数组划分成相等数对
给你一个整数数组 `nums` ，它包含 `2 * n` 个整数。

你需要将 `nums` 划分成 `n` 个数对，满足：
* 每个元素 **只属于一个** 数对。
* 同一数对中的元素 **相等** 。

如果可以将 `nums` 划分成 `n` 个数对，请你返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,2,3,2,2,2]
<strong>输出:</strong> true
<strong>解释:</strong>
nums 中总共有 6 个元素，所以它们应该被划分成 6 / 2 = 3 个数对。
nums 可以划分成 (2, 2) ，(3, 3) 和 (2, 2) ，满足所有要求。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> false
<strong>解释:</strong>
无法将 nums 划分成 4 / 2 = 2 个数对且满足所有要求。
</pre>

#### 提示:
* `nums.length == 2 * n`
* `1 <= n <= 500`
* `1 <= nums[i] <= 500`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        count.values().all(|v| v % 2 == 0)
    }
}
```
