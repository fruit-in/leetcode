# 2357. Make Array Zero by Subtracting Equal Amounts
You are given a non-negative integer array `nums`. In one operation, you must:
* Choose a positive integer `x` such that `x` is less than or equal to the **smallest non-zero** element in `nums`.
* Subtract `x` from every **positive** element in `nums`.

Return *the **minimum** number of operations to make every element in* `nums` *equal to* `0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,5,0,3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
In the first operation, choose x = 1. Now, nums = [0,4,0,2,4].
In the second operation, choose x = 2. Now, nums = [0,2,0,0,2].
In the third operation, choose x = 2. Now, nums = [0,0,0,0,0].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Each element in nums is already 0 so no operations are needed.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&&x| x > 0)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}
```
