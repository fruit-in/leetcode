# 2588. Count the Number of Beautiful Subarrays
You are given a **0-indexed** integer array `nums`. In one operation, you can:
* Choose two different indices `i` and `j` such that `0 <= i, j < nums.length`.
* Choose a non-negative integer `k` such that the <code>k<sup>th</sup></code> bit (**0-indexed**) in the binary representation of `nums[i]` and `nums[j]` is `1`.
* Subtract <code>2<sup>k</sup></code> from `nums[i]` and `nums[j]`.

A subarray is **beautiful** if it is possible to make all of its elements equal to `0` after applying the above operation any number of times (including zero).

Return *the number of **beautiful subarrays** in the array* `nums`.

A subarray is a contiguous **non-empty** sequence of elements within an array.

**Note:** Subarrays where all elements are initially 0 are considered beautiful, as no operation is needed.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,3,1,2,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 beautiful subarrays in nums: [4,3,1,2,4] and [4,3,1,2,4].
- We can make all elements in the subarray [3,1,2] equal to 0 in the following way:
  - Choose [3, 1, 2] and k = 1. Subtract 21 from both numbers. The subarray becomes [1, 1, 0].
  - Choose [1, 1, 0] and k = 0. Subtract 20 from both numbers. The subarray becomes [0, 0, 0].
- We can make all elements in the subarray [4,3,1,2,4] equal to 0 in the following way:
  - Choose [4, 3, 1, 2, 4] and k = 2. Subtract 22 from both numbers. The subarray becomes [0, 3, 1, 2, 0].
  - Choose [0, 3, 1, 2, 0] and k = 0. Subtract 20 from both numbers. The subarray becomes [0, 2, 0, 2, 0].
  - Choose [0, 2, 0, 2, 0] and k = 1. Subtract 21 from both numbers. The subarray becomes [0, 0, 0, 0, 0].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,10,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no beautiful subarrays in nums.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut mask = 0;
        let mut mask_count = HashMap::from([(0, 1)]);
        let mut ret = 0;

        for &x in &nums {
            mask ^= x;
            ret += mask_count.get(&mask).unwrap_or(&0);
            *mask_count.entry(mask).or_insert(0) += 1;
        }

        ret
    }
}
```
