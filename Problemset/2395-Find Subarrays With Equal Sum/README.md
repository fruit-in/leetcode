# 2395. Find Subarrays With Equal Sum
Given a **0-indexed** integer array `nums`, determine whether there exist **two** subarrays of length `2` with **equal** sum. Note that the two subarrays must begin at **different** indices.

Return `true` *if these subarrays exist, and* `false` *otherwise*.

A **subarray** is a contiguous non-empty sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,2,4]
<strong>Output:</strong> true
<strong>Explanation:</strong> The subarrays with elements [4,2] and [2,4] have the same sum of 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> No two subarrays of size 2 have the same sum.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0,0,0]
<strong>Output:</strong> true
<strong>Explanation:</strong> The subarrays [nums[0],nums[1]] and [nums[1],nums[2]] have the same sum of 0.
Note that even though the subarrays have the same content, the two subarrays are considered different because they are in different positions in the original array.
</pre>

#### Constraints:
* `2 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut subarray_sums = HashSet::new();

        for i in 1..nums.len() {
            if subarray_sums.contains(&(nums[i] + nums[i - 1])) {
                return true;
            }
            subarray_sums.insert(nums[i] + nums[i - 1]);
        }

        false
    }
}
```
