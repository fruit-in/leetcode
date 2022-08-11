# 930. Binary Subarrays With Sum
Given a binary array `nums` and an integer `goal`, return *the number of non-empty **subarrays** with a sum* `goal`.

A **subarray** is a contiguous part of the array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,0,1,0,1], goal = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 4 subarrays are bolded and underlined below:
[1,0,1,0,1]
[1,0,1,0,1]
[1,0,1,0,1]
[1,0,1,0,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0,0,0,0], goal = 0
<strong>Output:</strong> 15
</pre>

#### Constraints:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* `nums[i]` is either `0` or `1`.
* `0 <= goal <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut count = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;
        count.insert(0, 1);

        for num in nums {
            sum += num;
            ret += count.get(&(sum - goal)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
```
