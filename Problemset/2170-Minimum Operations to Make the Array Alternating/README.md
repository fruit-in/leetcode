# 2170. Minimum Operations to Make the Array Alternating
You are given a **0-indexed** array `nums` consisting of `n` positive integers.

The array `nums` is called **alternating** if:
* `nums[i - 2] == nums[i]`, where `2 <= i <= n - 1`.
* `nums[i - 1] != nums[i]`, where `1 <= i <= n - 1`.

In one **operation**, you can choose an index `i` and **change** `nums[i]` into **any** positive integer.

Return *the **minimum number of operations** required to make the array alternating*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,1,3,2,4,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
One way to make the array alternating is by converting it to [3,1,3,1,3,1].
The number of operations required in this case is 3.
It can be proven that it is not possible to make the array alternating in less than 3 operations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,2,2,2]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
One way to make the array alternating is by converting it to [1,2,1,2,1].
The number of operations required in this case is 2.
Note that the array cannot be converted to [2,2,2,2,2] because in this case nums[0] == nums[1] which violates the conditions of an alternating array.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut even_count = HashMap::new();
        let mut odd_count = HashMap::new();
        let mut even_max2 = [(0, 0), (0, 0)];
        let mut odd_max2 = [(0, 0), (0, 0)];

        for i in 0..n as usize {
            if i % 2 == 0 {
                *even_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == even_max2[0].0 {
                    even_max2[0].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[0].1 {
                    even_max2[1] = even_max2[0];
                    even_max2[0] = (nums[i], even_count[&nums[i]]);
                } else if nums[i] == even_max2[1].0 {
                    even_max2[1].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[1].1 {
                    even_max2[1] = (nums[i], even_count[&nums[i]]);
                }
            } else {
                *odd_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == odd_max2[0].0 {
                    odd_max2[0].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[0].1 {
                    odd_max2[1] = odd_max2[0];
                    odd_max2[0] = (nums[i], odd_count[&nums[i]]);
                } else if nums[i] == odd_max2[1].0 {
                    odd_max2[1].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[1].1 {
                    odd_max2[1] = (nums[i], odd_count[&nums[i]]);
                }
            }
        }

        if even_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                n / 2 - odd_max2[1].1
            } else {
                n / 2 - odd_max2[0].1
            }
        } else if odd_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                (n + 1) / 2 - even_max2[1].1
            } else {
                (n + 1) / 2 - even_max2[0].1
            }
        } else {
            if even_max2[0].0 == odd_max2[0].0 {
                n - (even_max2[0].1 + odd_max2[1].1).max(even_max2[1].1 + odd_max2[0].1)
            } else {
                n - even_max2[0].1 - odd_max2[0].1
            }
        }
    }
}
```
