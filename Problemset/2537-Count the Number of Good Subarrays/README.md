# 2537. Count the Number of Good Subarrays
Given an integer array `nums` and an integer `k`, return *the number of **good** subarrays of* `nums`.

A subarray `arr` is **good** if there are **at least** `k` pairs of indices `(i, j)` such that `i < j` and `arr[i] == arr[j]`.

A **subarray** is a contiguous **non-empty** sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1], k = 10
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only good subarray is the array nums itself.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,1,4,3,2,2,4], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 different good subarrays:
- [3,1,4,3,2,2] that has 2 pairs.
- [3,1,4,3,2,2,4] that has 3 pairs.
- [1,4,3,2,2,4] that has 2 pairs.
- [4,3,2,2,4] that has 2 pairs.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = HashMap::new();
        let mut pairs = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *count.entry(nums[j]).or_insert(0) += 1;
            pairs += count[&nums[j]] - 1;

            while pairs >= k {
                *count.get_mut(&nums[i]).unwrap() -= 1;
                pairs -= count[&nums[i]];
                i += 1;
            }

            ret += i as i64;
        }

        ret
    }
}
```
