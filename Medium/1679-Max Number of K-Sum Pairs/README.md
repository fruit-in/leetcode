# 1679. Max Number of K-Sum Pairs
You are given an integer array `nums` and an integer `k`.

In one operation, you can pick two numbers from the array whose sum equals `k` and remove them from the array.

Return *the maximum number of operations you can perform on the array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> Starting with nums = [1,2,3,4]:
- Remove numbers 1 and 4, then nums = [2,3]
- Remove numbers 2 and 3, then nums = []
There are no more pairs that sum up to 5, hence a total of 2 operations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,1,3,4,3], k = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> Starting with nums = [3,1,3,4,3]:
- Remove the first two 3's, then nums = [1,4,3]
There are no more pairs that sum up to 6, hence a total of 1 operation.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            if *count.get(&(k - num)).unwrap_or(&0) > 0 {
                *count.get_mut(&(k - num)).unwrap() -= 1;
                ret += 1;
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        ret
    }
}
```
