# 1031. Maximum Sum of Two Non-Overlapping Subarrays
Given an integer array `nums` and two integers `firstLen` and `secondLen`, return *the maximum sum of elements in two non-overlapping **subarrays** with lengths* `firstLen` *and* `secondLen`.

The array with length `firstLen` could occur before or after the array with length `secondLen`, but they have to be non-overlapping.

A **subarray** is a **contiguous** part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
<strong>Output:</strong> 20
<strong>Explanation:</strong> One choice of subarrays is [9] with length 1, and [6,5] with length 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
<strong>Output:</strong> 29
<strong>Explanation:</strong> One choice of subarrays is [3,8,1] with length 3, and [8,9] with length 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
<strong>Output:</strong> 31
<strong>Explanation:</strong> One choice of subarrays is [5,6,0,9] with length 4, and [0,3,8] with length 3.
</pre>

#### Constraints:
* `1 <= firstLen, secondLen <= 1000`
* `2 <= firstLen + secondLen <= 1000`
* `firstLen + secondLen <= nums.length <= 1000`
* `0 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = i32::MIN;

        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }

        for i in 0..nums.len() - first_len + 1 {
            let first_sum = prefix_sum[i + first_len] - prefix_sum[i];
            for j in 0..(i + 1).saturating_sub(second_len) {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
            for j in i + first_len..nums.len() - second_len + 1 {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
        }

        ret
    }
}
```
