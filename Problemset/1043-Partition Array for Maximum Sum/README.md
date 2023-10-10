# 1043. Partition Array for Maximum Sum
Given an integer array `arr`, partition the array into (contiguous) subarrays of length **at most** `k`. After partitioning, each subarray has their values changed to become the maximum value of that subarray.

Return *the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a **32-bit** integer*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,15,7,9,2,5,10], k = 3
<strong>Output:</strong> 84
<strong>Explanation:</strong> arr becomes [15,15,15,9,10,10,10]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
<strong>Output:</strong> 83
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1], k = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= arr.length <= 500`
* <code>0 <= arr[i] <= 10<sup>9</sup></code>
* `1 <= k <= arr.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];

        for i in 0..dp.len() {
            let mut max_val = 0;

            for j in 0..(k as usize).min(dp.len() - i - 1) {
                max_val = max_val.max(arr[i + j]);
                dp[i + j + 1] = dp[i + j + 1].max(dp[i] + max_val * (j as i32 + 1));
            }
        }

        *dp.last().unwrap()
    }
}
```
