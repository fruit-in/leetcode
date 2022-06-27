# 2006. Count Number of Pairs With Absolute Difference K
Given an integer array `nums` and an integer `k`, return *the number of pairs* `(i, j)` *where* `i < j` *such that* `|nums[i] - nums[j]| == k`.

The value of `|x|` is defined as:
* `x` if `x >= 0`.
* `-x` if `x < 0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,2,1], k = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> The pairs with an absolute difference of 1 are:
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3], k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no pairs with an absolute difference of 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,2,1,5,4], k = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> The pairs with an absolute difference of 2 are:
- [3,2,1,5,4]
- [3,2,1,5,4]
- [3,2,1,5,4]
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 100`
* `1 <= k <= 99`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            ret += count.get(&(num + k)).unwrap_or(&0);
            ret += count.get(&(num - k)).unwrap_or(&0);
            *count.entry(num).or_insert(0) += 1;
        }

        ret
    }
}
```
