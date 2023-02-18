# 2441. 与对应负数同时存在的最大正整数
给你一个 **不包含** 任何零的整数数组 `nums` ，找出自身与对应的负数都在数组中存在的最大正整数 `k` 。

返回正整数 `k` ，如果不存在这样的整数，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-1,2,-3,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 3 是数组中唯一一个满足题目要求的 k 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,10,6,7,-7,1]
<strong>输出:</strong> 7
<strong>解释:</strong> 数组中存在 1 和 7 对应的负数，7 的值更大。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-10,8,6,7,-2,-3]
<strong>输出:</strong> -1
<strong>解释:</strong> 不存在满足题目要求的 k ，返回 -1 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `-1000 <= nums[i] <= 1000`
* `nums[i] != 0`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let nums_set = nums.iter().collect::<HashSet<_>>();

        *nums
            .iter()
            .filter(|&&x| nums_set.contains(&-x))
            .max()
            .unwrap_or(&-1)
    }
}
```
