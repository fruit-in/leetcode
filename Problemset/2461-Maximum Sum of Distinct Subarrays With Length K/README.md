# 2461. Maximum Sum of Distinct Subarrays With Length K
You are given an integer array `nums` and an integer `k`. Find the maximum subarray sum of all the subarrays of `nums` that meet the following conditions:
* The length of the subarray is `k`, and
* All the elements of the subarray are **distinct**.

Return *the maximum subarray sum of all the subarrays that meet the conditions*. If no subarray meets the conditions, return `0`.

*A **subarray** is a contiguous non-empty sequence of elements within an array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,5,4,2,9,9,9], k = 3
<strong>Output:</strong> 15
<strong>Explanation:</strong> The subarrays of nums with length 3 are:
- [1,5,4] which meets the requirements and has a sum of 10.
- [5,4,2] which meets the requirements and has a sum of 11.
- [4,2,9] which meets the requirements and has a sum of 15.
- [2,9,9] which does not meet the requirements because the element 9 is repeated.
- [9,9,9] which does not meet the requirements because the element 9 is repeated.
We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,4,4], k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> The subarrays of nums with length 3 are:
- [4,4,4] which does not meet the requirements because the element 4 is repeated.
We return 0 because no subarrays meet the conditions.
</pre>

#### Constraints:
* <code>1 <= k <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut subarray_sum = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..k {
            subarray_sum += nums[j] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
        }
        if count.len() == k {
            ret = subarray_sum;
        }

        for j in k..nums.len() {
            subarray_sum += nums[j] as i64;
            subarray_sum -= nums[i] as i64;
            *count.entry(nums[j]).or_insert(0) += 1;
            *count.get_mut(&nums[i]).unwrap() -= 1;
            if count[&nums[i]] == 0 {
                count.remove(&nums[i]);
            }
            if count.len() == k {
                ret = ret.max(subarray_sum);
            }
            i += 1;
        }

        ret
    }
}
```
