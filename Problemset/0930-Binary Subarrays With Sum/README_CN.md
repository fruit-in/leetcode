# 930. 和相同的二元子数组
给你一个二元数组 `nums` ，和一个整数 `goal` ，请你统计并返回有多少个和为 `goal` 的 **非空** 子数组。

**子数组** 是数组的一段连续部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,0,1,0,1], goal = 2
<strong>输出:</strong> 4
<strong>解释:</strong>
有 4 个满足题目要求的子数组：[1,0,1]、[1,0,1,0]、[0,1,0,1]、[1,0,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,0,0,0], goal = 0
<strong>输出:</strong> 15
</pre>

#### 提示:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* `nums[i]` 不是 `0` 就是 `1`
* `0 <= goal <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut count = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;
        count.insert(0, 1);

        for num in nums {
            sum += num;
            ret += count.get(&(sum - goal)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
```
