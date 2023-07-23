# 2357. 使数组中所有元素都等于零
给你一个非负整数数组 `nums` 。在一步操作中，你必须：
* 选出一个正整数 `x` ，`x` 需要小于或等于 `nums` 中 **最小** 的 **非零** 元素。
* `nums` 中的每个正整数都减去 `x`。

返回使 `nums` 中所有元素都等于 `0` 需要的 **最少** 操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,5,0,3,5]
<strong>输出:</strong> 3
<strong>解释:</strong>
第一步操作：选出 x = 1 ，之后 nums = [0,4,0,2,4] 。
第二步操作：选出 x = 2 ，之后 nums = [0,2,0,0,2] 。
第三步操作：选出 x = 2 ，之后 nums = [0,0,0,0,0] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0]
<strong>输出:</strong> 0
<strong>解释:</strong> nums 中的每个元素都已经是 0 ，所以不需要执行任何操作。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&x| x > 0)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
```
