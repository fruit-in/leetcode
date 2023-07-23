# 2206. Divide Array Into Equal Pairs
You are given an integer array `nums` consisting of `2 * n` integers.

You need to divide `nums` into `n` pairs such that:
* Each element belongs to **exactly one** pair.
* The elements present in a pair are **equal**.

Return `true` *if nums can be divided into* `n` *pairs, otherwise return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,2,3,2,2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong>
There are 6 elements in nums, so they should be divided into 6 / 2 = 3 pairs.
If nums is divided into the pairs (2, 2), (3, 3), and (2, 2), it will satisfy all the conditions.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> false
<strong>Explanation:</strong>
There is no way to divide nums into 4 / 2 = 2 pairs such that the pairs satisfy every condition.
</pre>

#### Constraints:
* `nums.length == 2 * n`
* `1 <= n <= 500`
* `1 <= nums[i] <= 500`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        count.values().all(|v| v % 2 == 0)
    }
}
```
