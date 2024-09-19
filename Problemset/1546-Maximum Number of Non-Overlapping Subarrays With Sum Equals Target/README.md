# 1546. Maximum Number of Non-Overlapping Subarrays With Sum Equals Target
Given an array `nums` and an integer `target`, return *the maximum number of **non-empty non-overlapping** subarrays such that the sum of values in each subarray is equal to* `target`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], target = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 non-overlapping subarrays [1,1,1,1,1] with sum equals to target(2).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1,3,5,1,4,2,-9], target = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 3 subarrays with sum equal to 6.
([5,1], [4,2], [3,5,1,4,2,-9]) but only the first 2 are non-overlapping.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>
* <code>0 <= target <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
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
