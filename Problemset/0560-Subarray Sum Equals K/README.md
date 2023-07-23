# 560. Subarray Sum Equals K
Given an array of integers `nums` and an integer `k`, return *the total number of continuous subarrays whose sum equals to `k`*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,1], k = 2
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3], k = 3
<strong>Output:</strong> 2
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* `-1000 <= nums[i] <= 1000`
* <code>-10<sup>7</sup> <= k <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for num in nums {
            sum += num;
            ret += count.get(&(sum - k)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
```
