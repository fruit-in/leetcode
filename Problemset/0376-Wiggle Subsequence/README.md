# 376. Wiggle Subsequence
A **wiggle sequence** is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.

* For example, `[1, 7, 4, 9, 2, 5]` is a **wiggle sequence** because the differences `(6, -3, 5, -7, 3)` alternate between positive and negative.
* In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.

A **subsequence** is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.

Given an integer array `nums`, return *the length of the longest **wiggle subsequence** of* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,7,4,9,2,5]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,17,5,10,13,15,10,5,16,8]
<strong>Output:</strong> 7
<strong>Explanation:</strong> There are several subsequences that achieve this length.
One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7,8,9]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `0 <= nums[i] <= 1000`

**Follow up:** Could you solve this in `O(n)` time?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut asc = nums[(1..nums.len()).find(|&i| nums[i] != nums[0]).unwrap_or(0)] < nums[0];
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] && (asc ^ (nums[i] > nums[i - 1])) {
                asc = !asc;
                ret += 1;
            }
        }

        ret
    }
}
```
