# 15. 3Sum
Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.

Notice that the solution set must not contain duplicate triplets.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-1,0,1,2,-1,-4]
<strong>Output:</strong> [[-1,-1,2],[-1,0,1]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = []
<strong>Output:</strong> []
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> []
</pre>

#### Constraints:
* `0 <= nums.length <= 3000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut triplets = HashSet::new();
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                if nums[i] + nums[j] + nums[k] == 0 {
                    triplets.insert(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                } else if nums[i] + nums[j] + nums[k] < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        triplets.into_iter().collect()
    }
}
```
