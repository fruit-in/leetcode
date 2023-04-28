# 2364. Count Number of Bad Pairs
You are given a **0-indexed** integer array `nums`. A pair of indices `(i, j)` is a **bad pair** if `i < j` and `j - i != nums[j] - nums[i]`.

Return *the total number of **bad pairs** in* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,1,3,3]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The pair (0, 1) is a bad pair since 1 - 0 != 1 - 4.
The pair (0, 2) is a bad pair since 2 - 0 != 3 - 4, 2 != -1.
The pair (0, 3) is a bad pair since 3 - 0 != 3 - 4, 3 != -1.
The pair (1, 2) is a bad pair since 2 - 1 != 3 - 1, 1 != 2.
The pair (2, 3) is a bad pair since 3 - 2 != 3 - 3, 1 != 0.
There are a total of 5 bad pairs, so we return 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no bad pairs.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            ret += i as i64 - count.get(&(i as i32 - nums[i])).unwrap_or(&0);
            count
                .entry(i as i32 - nums[i])
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        ret
    }
}
```
