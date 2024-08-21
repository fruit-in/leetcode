# 659. 分割数组为连续子序列
给你一个按 **非递减顺序** 排列的整数数组 `nums` 。

请你判断是否能在将 `nums` 分割成 **一个或多个子序列** 的同时满足下述 **两个** 条件：

* 每个子序列都是一个 **连续递增序列**（即，每个整数 **恰好** 比前一个整数大 **1** ）。
* 所有子序列的长度 **至少** 为 `3` 。

如果可以分割 `nums` 并满足上述条件，则返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,3,4,5]
<strong>输出:</strong> true
<strong>解释:</strong> nums 可以分割成以下子序列：
[1,2,3,3,4,5] --> 1, 2, 3
[1,2,3,3,4,5] --> 3, 4, 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,3,4,4,5,5]
<strong>输出:</strong> true
<strong>解释:</strong> nums 可以分割成以下子序列：
[1,2,3,3,4,4,5,5] --> 1, 2, 3, 4, 5
[1,2,3,3,4,4,5,5] --> 3, 4, 5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,4,5]
<strong>输出:</strong> false
<strong>解释:</strong> 无法将 nums 分割成长度至少为 3 的连续递增子序列。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `-1000 <= nums[i] <= 1000`
* `nums` 按非递减顺序排列

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for &num in &nums {
            let (a, b, c) = *count.get(&(num - 1)).unwrap_or(&(0, 0, 0));
            let (d, e, f) = *count.get(&num).unwrap_or(&(0, 0, 0));

            if a > 0 {
                count.insert(num - 1, (a - 1, b, c));
                count.insert(num, (d, e + 1, f));
            } else if b > 0 {
                count.insert(num - 1, (a, b - 1, c));
                count.insert(num, (d, e, f + 1));
            } else if c > 0 {
                count.insert(num - 1, (a, b, c - 1));
                count.insert(num, (d, e, f + 1));
            } else {
                count.insert(num, (d + 1, e, f));
            }
        }

        count.values().all(|&(a, b, _)| a | b == 0)
    }
}
```
