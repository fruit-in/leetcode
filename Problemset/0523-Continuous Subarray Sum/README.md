# 523. Continuous Subarray Sum
Given an integer array nums and an integer k, return `true` *if* `nums` *has a **good subarray** or* `false` *otherwise*.

A **good subarray** is a subarray where:
* its length is **at least two**, and
* the sum of the elements of the subarray is a multiple of `k`.

**Note** that:
* A **subarray** is a contiguous part of the array.
* An integer `x` is a multiple of `k` if there exists an integer `n` such that `x = n * k`. `0` is **always** a multiple of `k`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [23,2,4,6,7], k = 6
<strong>Output:</strong> true
<strong>Explanation:</strong> [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [23,2,6,4,7], k = 6
<strong>Output:</strong> true
<strong>Explanation:</strong> [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [23,2,6,4,7], k = 13
<strong>Output:</strong> false
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= sum(nums[i]) <= 2<sup>31</sup> - 1</code>
* <code>1 <= k <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_mod = 0;
        let mut mod_indices = HashMap::from([(0, -1)]);

        for i in 0..nums.len() {
            prefix_mod = (prefix_mod + nums[i]) % k;

            match mod_indices.get(&prefix_mod) {
                Some(j) if j + 1 < i as i32 => return true,
                None => {
                    mod_indices.insert(prefix_mod, i as i32);
                }
                _ => (),
            }
        }

        false
    }
}
```
