# 2547. Minimum Cost to Split an Array
You are given an integer array `nums` and an integer `k`.

Split the array into some number of non-empty subarrays. The **cost** of a split is the sum of the **importance value** of each subarray in the split.

Let `trimmed(subarray)` be the version of the subarray where all numbers which appear only once are removed.

* For example, `trimmed([3,1,2,4,3,4]) = [3,4,3,4]`.

The **importance value** of a subarray is `k + trimmed(subarray).length`.

* For example, if a subarray is `[1,2,3,3,3,4,4]`, then trimmed(`[1,2,3,3,3,4,4]) = [3,3,3,4,4].`The importance value of this subarray will be `k + 5`.

Return *the minimum possible cost of a split of* `nums`.

A **subarray** is a contiguous **non-empty** sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,1,2,1,3,3], k = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> We split nums to have two subarrays: [1,2], [1,2,1,3,3].
The importance value of [1,2] is 2 + (0) = 2.
The importance value of [1,2,1,3,3] is 2 + (2 + 2) = 6.
The cost of the split is 2 + 6 = 8. It can be shown that this is the minimum possible cost among all the possible splits.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,1,2,1], k = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> We split nums to have two subarrays: [1,2], [1,2,1].
The importance value of [1,2] is 2 + (0) = 2.
The importance value of [1,2,1] is 2 + (2) = 4.
The cost of the split is 2 + 4 = 6. It can be shown that this is the minimum possible cost among all the possible splits.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,1,2,1], k = 5
<strong>Output:</strong> 10
<strong>Explanation:</strong> We split nums to have one subarray: [1,2,1,2,1].
The importance value of [1,2,1,2,1] is 5 + (3 + 2) = 10.
The cost of the split is 10. It can be shown that this is the minimum possible cost among all the possible splits.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `0 <= nums[i] < nums.length`
* <code>1 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut dp = vec![i64::MAX; nums.len() + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            let mut count = HashMap::new();
            let mut trimmed_length = 0;

            for j in (0..i).rev() {
                *count.entry(nums[j]).or_insert(0) += 1;
                trimmed_length += match count.get(&nums[j]).unwrap() {
                    &1 => 0,
                    &2 => 2,
                    _ => 1,
                };
                dp[i] = dp[i].min(dp[j] + k + trimmed_length);
            }
        }

        *dp.last().unwrap() as i32
    }
}
```
