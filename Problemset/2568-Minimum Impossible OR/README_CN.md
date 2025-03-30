# 2568. 最小无法得到的或值
给你一个下标从 **0** 开始的整数数组 `nums` 。

如果存在一些整数满足 <code>0 <= index<sub>1</sub> < index<sub>2</sub> < ... < index<sub>k</sub> < nums.length</code> ，得到 <code>nums[index<sub>1</sub>] | nums[index<sub>2</sub>] | ... | nums[index<sub>k</sub>] = x</code> ，那么我们说 `x` 是 可表达的 。换言之，如果一个整数能由 `nums` 的某个子序列的或运算得到，那么它就是可表达的。

请你返回 `nums` 不可表达的 最小非零整数 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 1 和 2 已经在数组中，因为 nums[0] | nums[1] = 2 | 1 = 3 ，所以 3 是可表达的。由于 4 是不可表达的，所以我们返回 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,3,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 1 是最小不可表达的数字。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();

        for i in 0..32 {
            if !nums.contains(&(1 << i)) {
                return 1 << i;
            }
        }

        unreachable!()
    }
}
```
