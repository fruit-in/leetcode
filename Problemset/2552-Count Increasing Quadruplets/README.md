# 2552. Count Increasing Quadruplets
Given a **0-indexed** integer array `nums` of size `n` containing all numbers from `1` to `n`, return *the number of increasing quadruplets*.

A quadruplet `(i, j, k, l)` is increasing if:
* `0 <= i < j < k < l < n`, and
* `nums[i] < nums[k] < nums[j] < nums[l]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,2,4,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- When i = 0, j = 1, k = 2, and l = 3, nums[i] < nums[k] < nums[j] < nums[l].
- When i = 0, j = 1, k = 2, and l = 4, nums[i] < nums[k] < nums[j] < nums[l].
There are no other quadruplets, so we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There exists only one quadruplet with i = 0, j = 1, k = 2, l = 3, but since nums[j] < nums[k], we return 0.
</pre>

#### Constraints:
* `4 <= nums.length <= 4000`
* `1 <= nums[i] <= nums.length`
* All the integers of `nums` are **unique**. `nums` is a permutation.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let mut n = nums.len();
        let mut counti = HashMap::new();
        let mut ret = 0;

        for k in 2..n - 1 {
            let mut count = 0;

            for j in 0..k {
                if nums[j] > nums[k] && count > 0 {
                    counti.insert((j, k), count);
                } else if nums[j] < nums[k] {
                    count += 1;
                }
            }
        }

        for j in 1..n - 2 {
            let mut count = 0;

            for k in (j + 1..n).rev() {
                if nums[k] < nums[j] && count > 0 {
                    ret += count * counti.get(&(j, k)).unwrap_or(&0);
                } else if nums[k] > nums[j] {
                    count += 1;
                }
            }
        }

        ret
    }
}
```
