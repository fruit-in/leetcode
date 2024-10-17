# 1911. Maximum Alternating Subsequence Sum
The **alternating sum** of a **0-indexed** array is defined as the **sum** of the elements at **even** indices **minus** the **sum** of the elements at **odd** indices.

* For example, the alternating sum of `[4,2,5,3]` is `(4 + 5) - (2 + 3) = 4`.

Given an array `nums`, return *the **maximum alternating sum** of any subsequence of* `nums` *(after **reindexing** the elements of the subsequence)*.

A **subsequence** of an array is a new array generated from the original array by deleting some elements (possibly none) without changing the remaining elements' relative order. For example, `[2,7,4]` is a subsequence of `[4,2,3,7,2,1,4]` (the underlined elements), while `[2,4,2]` is not.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,2,5,3]
<strong>Output:</strong> 7
<strong>Explanation:</strong> It is optimal to choose the subsequence [4,2,5] with alternating sum (4 + 5) - 2 = 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,6,7,8]
<strong>Output:</strong> 8
<strong>Explanation:</strong> It is optimal to choose the subsequence [8] with alternating sum 8.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [6,2,1,2,4,5]
<strong>Output:</strong> 10
<strong>Explanation:</strong> It is optimal to choose the subsequence [6,1,5] with alternating sum (6 + 5) - 1 = 10.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut dp = vec![[0; 2]; nums.len() + 1];

        for i in 0..nums.len() {
            dp[i + 1][0] = dp[i][0].max(dp[i][1] + nums[i] as i64);
            dp[i + 1][1] = dp[i][1].max(dp[i][0] - nums[i] as i64);
        }

        dp.last().unwrap()[0]
    }
}
```
