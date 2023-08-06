# 410. Split Array Largest Sum
Given an integer array `nums` and an integer `k`, split `nums` into `k` non-empty subarrays such that the largest sum of any subarray is **minimized**.

Return *the minimized largest sum of the split*.

A **subarray** is a contiguous part of the array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [7,2,5,10,8], k = 2
<strong>Output:</strong> 18
<strong>Explanation:</strong> There are four ways to split nums into two subarrays.
The best way is to split it into [7,2,5] and [10,8], where the largest sum among the two subarrays is only 18.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5], k = 2
<strong>Output:</strong> 9
<strong>Explanation:</strong> There are four ways to split nums into two subarrays.
The best way is to split it into [1,2,3] and [4,5], where the largest sum among the two subarrays is only 9.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>0 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= min(50, nums.length)`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![vec![0; k as usize + 1]; nums.len() + 1];

        for i in 1..=nums.len() {
            dp[i][1] = dp[i - 1][1] + nums[i - 1];
        }

        for i in 2..=nums.len() {
            for j in 2..=i.min(k as usize) {
                dp[i][j] = i32::MAX;
                for x in j - 1..i {
                    dp[i][j] = dp[i][j].min(dp[x][j - 1].max(dp[i][1] - dp[x][1]));
                }
            }
        }

        dp[nums.len()][k as usize]
    }
}
```
