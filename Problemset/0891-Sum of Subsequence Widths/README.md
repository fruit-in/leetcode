# 891. Sum of Subsequence Widths
The **width** of a sequence is the difference between the maximum and minimum elements in the sequence.

Given an array of integers `nums`, return *the sum of the **widths** of all the non-empty **subsequences** of* `nums`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **subsequence** is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, `[3,6,2,7]` is a subsequence of the array `[0,3,1,6,2,2,7]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The subsequences are [1], [2], [3], [2,1], [2,3], [1,3], [2,1,3].
The corresponding widths are 0, 0, 0, 1, 1, 2, 2.
The sum of these widths is 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut pow2 = 1;
        let mut pow2_sum = 1;
        let mut ret = 0;

        nums.sort_unstable();

        for i in 1..nums.len() {
            x = (2 * x + (nums[i] - nums[i - 1]) as i64 * pow2_sum) % 1_000_000_007;
            pow2 = (2 * pow2) % 1_000_000_007;
            pow2_sum = (pow2_sum + pow2) % 1_000_000_007;
            ret = (ret + x) % 1_000_000_007;
        }

        ret as i32
    }
}
```
