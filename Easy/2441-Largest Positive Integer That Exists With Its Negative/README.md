# 2441. Largest Positive Integer That Exists With Its Negative
Given an integer array `nums` that **does not contain** any zeros, find **the largest positive** integer `k` such that `-k` also exists in the *array*.

Return *the positive integer* `k`. If there is no such integer, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-1,2,-3,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 3 is the only valid k we can find in the array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1,10,6,7,-7,1]
<strong>Output:</strong> 7
<strong>Explanation:</strong> Both 1 and 7 have their corresponding negative values in the array. 7 has a larger value.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [-10,8,6,7,-2,-3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no a single valid k, we return -1.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `-1000 <= nums[i] <= 1000`
* `nums[i] != 0`

## Solutions (Rust)

### 1. Solution
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
