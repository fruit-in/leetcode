# 2568. Minimum Impossible OR
You are given a **0-indexed** integer array `nums`.

We say that an integer x is **expressible** from `nums` if there exist some integers <code>0 <= index<sub>1</sub> < index<sub>2</sub> < ... < index<sub>k</sub> < nums.length</code> for which <code>nums[index<sub>1</sub>] | nums[index<sub>2</sub>] | ... | nums[index<sub>k</sub>] = x</code>. In other words, an integer is expressible if it can be written as the bitwise OR of some subsequence of `nums`.

Return *the minimum **positive non-zero integer** that is not expressible from* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> 1 and 2 are already present in the array. We know that 3 is expressible, since nums[0] | nums[1] = 2 | 1 = 3. Since 4 is not expressible, we return 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,3,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can show that 1 is the smallest number that is not expressible.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();

        for i in 0..32 {
            if !nums.contains(&(1 << i)) {
                return 1 << i;
            }
        }

        unreachable!()
    }
}
```
