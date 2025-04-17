# 2302. Count Subarrays With Score Less Than K
The **score** of an array is defined as the **product** of its sum and its length.

* For example, the score of `[1, 2, 3, 4, 5]` is `(1 + 2 + 3 + 4 + 5) * 5 = 75`.

Given a positive integer array `nums` and an integer `k`, return *the **number of non-empty subarrays** of* `nums` *whose score is **strictly less** than* `k`.

A **subarray** is a contiguous sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,4,3,5], k = 10
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The 6 subarrays having scores less than 10 are:
- [2] with score 2 * 1 = 2.
- [1] with score 1 * 1 = 1.
- [4] with score 4 * 1 = 4.
- [3] with score 3 * 1 = 3.
- [5] with score 5 * 1 = 5.
- [2,1] with score (2 + 1) * 2 = 6.
Note that subarrays such as [1,4] and [4,3,5] are not considered because their scores are 10 and 36 respectively, while we need scores strictly less than 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1], k = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Every subarray except [1,1,1] has a score less than 5.
[1,1,1] has a score (1 + 1 + 1) * 3 = 9, which is greater than 5.
Thus, there are 5 subarrays having scores less than 5.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        for i in 0..nums.len() {
            let mut lo = 0;
            let mut hi = i + 1;

            while lo < hi {
                let mid = (lo + hi) / 2;
                let score = (prefix_sum[i + 1] - prefix_sum[mid]) * (i + 1 - mid) as i64;

                if score < k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            ret += (i + 1 - hi) as i64;
        }

        ret
    }
}
```
