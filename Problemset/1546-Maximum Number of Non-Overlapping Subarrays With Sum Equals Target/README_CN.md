# 1546. 和为目标值且不重叠的非空子数组的最大数目
给你一个数组 `nums` 和一个整数 `target` 。

请你返回 **非空不重叠** 子数组的最大数目，且每个子数组中数字和都为 `target` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], target = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 总共有 2 个不重叠子数组（加粗数字表示） [1,1,1,1,1] ，它们的和为目标值 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,3,5,1,4,2,-9], target = 6
<strong>输出:</strong> 2
<strong>解释:</strong> 总共有 3 个子数组和为 6 。
([5,1], [4,2], [3,5,1,4,2,-9]) 但只有前 2 个是不重叠的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-2,6,6,3,5,4,1,2,8], target = 10
<strong>输出:</strong> 3
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [0,0,0], target = 0
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>
* <code>0 <= target <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut last_index = HashMap::from([(0, 0)]);
        let mut max_count = vec![0; nums.len() + 1];

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
            if let Some(&j) = last_index.get(&(prefix_sum[i + 1] - target)) {
                max_count[i + 1] = max_count[j] + 1;
            }
            max_count[i + 1] = max_count[i + 1].max(max_count[i]);
            last_index.insert(prefix_sum[i + 1], i + 1);
        }

        *max_count.last().unwrap()
    }
}
```
