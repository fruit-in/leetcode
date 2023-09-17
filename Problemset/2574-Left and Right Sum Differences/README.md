# 2574. Left and Right Sum Differences
Given a **0-indexed** integer array `nums`, find a **0-indexed** integer array `answer` where:

* `answer.length == nums.length`.
* `answer[i] = |leftSum[i] - rightSum[i]|`.

Where:

* `leftSum[i]` is the sum of elements to the left of the index `i` in the array `nums`. If there is no such element, `leftSum[i] = 0`.
* `rightSum[i]` is the sum of elements to the right of the index `i` in the array `nums`. If there is no such element, `rightSum[i] = 0`.

Return *the array* `answer`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,4,8,3]
<strong>Output:</strong> [15,1,11,22]
<strong>Explanation:</strong> The array leftSum is [0,10,14,22] and the array rightSum is [15,11,3,0].
The array answer is [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> [0]
<strong>Explanation:</strong> The array leftSum is [0] and the array rightSum is [0].
The array answer is [|0 - 0|] = [0].
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];

        for i in 1..n {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
            right_sum[n - 1 - i] = right_sum[n - i] + nums[n - i];
        }

        (0..n).map(|i| (left_sum[i] - right_sum[i]).abs()).collect()
    }
}
```
