# 2542. Maximum Subsequence Score
You are given two **0-indexed** integer arrays `nums1` and `nums2` of equal length `n` and a positive integer `k`. You must choose a **subsequence** of indices from `nums1` of length `k`.

For chosen indices <code>i<sub>0</sub></code>, <code>i<sub>1</sub></code>, ..., <code>i<sub>k - 1</sub></code>, your **score** is defined as:
* The sum of the selected elements from `nums1` multiplied with the **minimum** of the selected elements from `nums2`.
* It can defined simply as: <code>(nums1[i<sub>0</sub>] + nums1[i<sub>1</sub>] +...+ nums1[i<sub>k - 1</sub>]) * min(nums2[i<sub>0</sub>] , nums2[i<sub>1</sub>], ... ,nums2[i<sub>k - 1</sub>])</code>.

Return *the **maximum** possible score*.

A **subsequence** of indices of an array is a set that can be derived from the set `{0, 1, ..., n-1}` by deleting some or no elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,3,3,2], nums2 = [2,1,3,4], k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong>
The four possible subsequence scores are:
- We choose the indices 0, 1, and 2 with score = (1+3+3) * min(2,1,3) = 7.
- We choose the indices 0, 1, and 3 with score = (1+3+2) * min(2,1,4) = 6.
- We choose the indices 0, 2, and 3 with score = (1+3+2) * min(2,3,4) = 12.
- We choose the indices 1, 2, and 3 with score = (3+3+2) * min(1,3,4) = 8.
Therefore, we return the max score, which is 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [4,2,3,1,1], nums2 = [7,5,10,9,6], k = 1
<strong>Output:</strong> 30
<strong>Explanation:</strong>
Choosing index 2 is optimal: nums1[2] * nums2[2] = 3 * 10 = 30 is the maximum possible score.
</pre>

#### Constraints:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* `1 <= k <= n`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        let mut ret = 0;

        nums.sort_unstable_by_key(|&(_, y)| -y);

        for i in 0..nums.len() {
            heap.push(-nums[i].0);
            sum += *nums[i].0 as i64;

            if i >= k - 1 {
                if heap.len() > k {
                    sum -= -heap.pop().unwrap() as i64;
                }
                ret = ret.max(sum * *nums[i].1 as i64);
            }
        }

        ret
    }
}
```
