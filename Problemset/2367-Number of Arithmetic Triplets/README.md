# 2367. Number of Arithmetic Triplets
You are given a **0-indexed**, **strictly increasing** integer array `nums` and a positive integer `diff`. A triplet `(i, j, k)` is an **arithmetic triplet** if the following conditions are met:

* `i < j < k`,
* `nums[j] - nums[i] == diff`, and
* `nums[k] - nums[j] == diff`.

Return *the number of unique **arithmetic triplets***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,4,6,7,10], diff = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong>
(1, 2, 4) is an arithmetic triplet because both 7 - 4 == 3 and 4 - 1 == 3.
(2, 4, 5) is an arithmetic triplet because both 10 - 7 == 3 and 7 - 4 == 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,5,6,7,8,9], diff = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
(0, 2, 4) is an arithmetic triplet because both 8 - 6 == 2 and 6 - 4 == 2.
(1, 3, 5) is an arithmetic triplet because both 9 - 7 == 2 and 7 - 5 == 2.
</pre>

#### Constraints:
* `3 <= nums.length <= 200`
* `0 <= nums[i] <= 200`
* `1 <= diff <= 50`
* `nums` is **strictly** increasing.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut nums_set = HashSet::from([nums[0], nums[1]]);
        let mut ret = 0;

        for &num in &nums[2..] {
            nums_set.insert(num);

            if nums_set.contains(&(num - diff)) && nums_set.contains(&(num - 2 * diff)) {
                ret += 1;
            }
        }

        ret
    }
}
```
