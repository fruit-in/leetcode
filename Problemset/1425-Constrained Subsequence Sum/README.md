# 1425. Constrained Subsequence Sum
Given an integer array `nums` and an integer `k`, return the maximum sum of a **non-empty** subsequence of that array such that for every two **consecutive** integers in the subsequence, `nums[i]` and `nums[j]`, where `i < j`, the condition `j - i <= k` is satisfied.

A *subsequence* of an array is obtained by deleting some number of elements (can be zero) from the array, leaving the remaining elements in their original order.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,2,-10,5,20], k = 2
<strong>Output:</strong> 37
<strong>Explanation:</strong> The subsequence is [10, 2, 5, 20].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1,-2,-3], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> The subsequence must be non-empty, so we choose the largest number.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [10,-2,-10,-5,20], k = 2
<strong>Output:</strong> 23
<strong>Explanation:</strong> The subsequence is [10, -2, -5, 20].
</pre>

#### Constraints:
* <code>1 <= k <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut ret = *nums.iter().max().unwrap();

        for i in 0..nums.len() {
            if i - deque.front().unwrap_or(&(i, 0)).0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque.front().unwrap_or(&(0, 0)).1;

            if x > 0 {
                while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                    deque.pop_back();
                }
                deque.push_back((i, x));
                ret = ret.max(x);
            }
        }

        ret
    }
}
```
