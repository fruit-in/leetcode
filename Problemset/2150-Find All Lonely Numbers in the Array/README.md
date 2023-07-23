# 2150. Find All Lonely Numbers in the Array
You are given an integer array `nums`. A number `x` is **lonely** when it appears only **once**, and no **adjacent** numbers (i.e. `x + 1` and `x - 1`) appear in the array.

Return ***all** lonely numbers in* `nums`. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,6,5,8]
<strong>Output:</strong> [10,8]
<strong>Explanation:</strong>
- 10 is a lonely number since it appears exactly once and 9 and 11 does not appear in nums.
- 8 is a lonely number since it appears exactly once and 7 and 9 does not appear in nums.
- 5 is not a lonely number since 6 appears in nums and vice versa.
Hence, the lonely numbers in nums are [10, 8].
Note that [8, 10] may also be returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3,5,3]
<strong>Output:</strong> [1,5]
<strong>Explanation:</strong>
- 1 is a lonely number since it appears exactly once and 0 and 2 does not appear in nums.
- 5 is a lonely number since it appears exactly once and 4 and 6 does not appear in nums.
- 3 is not a lonely number since it appears twice.
Hence, the lonely numbers in nums are [1, 5].
Note that [5, 1] may also be returned.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut is_lonely = HashMap::new();

        for &n in &nums {
            is_lonely
                .entry(n)
                .and_modify(|b| *b = false)
                .or_insert(true);
            is_lonely.insert(n + 1, false);
            is_lonely.insert(n - 1, false);
        }

        nums.into_iter().filter(|n| is_lonely[n]).collect()
    }
}
```
