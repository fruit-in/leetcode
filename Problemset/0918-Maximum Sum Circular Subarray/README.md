# 918. Maximum Sum Circular Subarray
Given a **circular integer array** `nums` of length `n`, return *the maximum possible sum of a non-empty **subarray** of* `nums`.

A **circular array** means the end of the array connects to the beginning of the array. Formally, the next element of `nums[i]` is `nums[(i + 1) % n]` and the previous element of `nums[i]` is `nums[(i - 1 + n) % n]`.

A **subarray** may only include each element of the fixed buffer `nums` at most once. Formally, for a subarray `nums[i], nums[i + 1], ..., nums[j]`, there does not exist `i <= k1`, `k2 <= j` with `k1 % n == k2 % n`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,-2,3,-2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Subarray [3] has maximum sum 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,-3,5]
<strong>Output:</strong> 10
<strong>Explanation:</strong> Subarray [5,5] has maximum sum 5 + 5 = 10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [-3,-2,-3]
<strong>Output:</strong> -2
<strong>Explanation:</strong> Subarray [-2] has maximum sum -2.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* <code>-3 * 10<sup>4</sup> <= nums[i] <= 3 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum();
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut prefix_sum = nums[0];
        let mut ret = nums[0].max(sum);

        for i in 1..nums.len() {
            prefix_sum += nums[i];
            ret = ret
                .max(prefix_sum - min_sum)
                .max(sum - prefix_sum + max_sum);
            max_sum = max_sum.max(prefix_sum);
            min_sum = min_sum.min(prefix_sum);
        }

        ret
    }
}
```
