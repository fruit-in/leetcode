# 992. Subarrays with K Different Integers
Given an integer array `nums` and an integer `k`, return *the number of **good subarrays** of* `nums`.

A **good array** is an array where the number of different integers in that array is exactly `k`.

* For example, `[1,2,3,1,2]` has `3` different integers: `1`, `2`, and `3`.

A **subarray** is a **contiguous** part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,1,2,3], k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,1,3,4], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* `1 <= nums[i], k <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut max_index = HashMap::new();
        let mut l = 0;
        let mut m = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            *max_index.entry(nums[r]).or_insert(r) = r;

            while count.len() > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                if *count.get(&nums[l]).unwrap() == 0 {
                    count.remove(&nums[l]);
                }
                l += 1;
            }

            if count.len() == k {
                while m < l || max_index[&nums[m]] != m {
                    m += 1;
                }
                ret += m - l + 1;
            }
        }

        ret as i32
    }
}
```
