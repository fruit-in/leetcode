# 2475. Number of Unequal Triplets in Array
You are given a **0-indexed** array of positive integers `nums`. Find the number of triplets `(i, j, k)` that meet the following conditions:

* `0 <= i < j < k < nums.length`
* `nums[i]`, `nums[j]`, and `nums[k]` are **pairwise distinct**.
    * In other words, `nums[i] != nums[j]`, `nums[i] != nums[k]`, and `nums[j] != nums[k]`.

Return *the number of triplets that meet the conditions*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,4,2,4,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The following triplets meet the conditions:
- (0, 2, 4) because 4 != 2 != 3
- (1, 2, 4) because 4 != 2 != 3
- (2, 3, 4) because 2 != 4 != 3
Since there are 3 triplets, we return 3.
Note that (2, 0, 4) is not a valid triplet because 2 > 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> No triplets meet the conditions so we return 0.
</pre>

#### Constraints:
* `3 <= nums.length <= 100`
* `1 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in &nums {
            count.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        let count = count.values().collect::<Vec<_>>();

        for i in 0..count.len() {
            for j in i + 1..count.len() {
                for k in j + 1..count.len() {
                    ret += count[i] * count[j] * count[k];
                }
            }
        }

        ret
    }
}
```
