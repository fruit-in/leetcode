# 2567. Minimum Score by Changing Two Elements
You are given an integer array `nums`.

* The **low** score of `nums` is the **minimum** absolute difference between any two integers.
* The **high** score of `nums` is the **maximum** absolute difference between any two integers.
* The **score** of `nums` is the sum of the **high** and **low** scores.

Return the **minimum score** after **changing two elements** of `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,4,7,8,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Change nums[0] and nums[1] to be 6 so that nums becomes [6,6,7,8,5].
The low score is the minimum absolute difference: |6 - 6| = 0.
The high score is the maximum absolute difference: |8 - 5| = 3.
The sum of high and low score is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,4,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Change nums[1] and nums[2] to 1 so that nums becomes [1,1,1].
The sum of maximum absolute difference and minimum absolute difference is 0.
</pre>

#### Constraints:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.sort_unstable();

        (nums[n - 1] - nums[2])
            .min(nums[n - 2] - nums[1])
            .min(nums[n - 3] - nums[0])
    }
}
```
