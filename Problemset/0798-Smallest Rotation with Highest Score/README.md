# 798. Smallest Rotation with Highest Score
You are given an array `nums`. You can rotate it by a non-negative integer `k` so that the array becomes `[nums[k], nums[k + 1], ... nums[nums.length - 1], nums[0], nums[1], ..., nums[k-1]]`. Afterward, any entries that are less than or equal to their index are worth one point.

* For example, if we have `nums = [2,4,1,3,0]`, and we rotate by `k = 2`, it becomes `[1,3,0,2,4]`. This is worth `3` points because `1 > 0` [no points], `3 > 1` [no points], `0 <= 2` [one point], `2 <= 3` [one point], `4 <= 4` [one point].

Return *the rotation index* `k` *that corresponds to the highest score we can achieve if we rotated* `nums` *by it*. If there are multiple answers, return the smallest such index `k`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,1,4,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Scores for each k are listed below:
k = 0,  nums = [2,3,1,4,0],    score 2
k = 1,  nums = [3,1,4,0,2],    score 3
k = 2,  nums = [1,4,0,2,3],    score 3
k = 3,  nums = [4,0,2,3,1],    score 4
k = 4,  nums = [0,2,3,1,4],    score 3
So we should choose k = 3, which has the highest score.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3,0,2,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums will always have 3 points no matter how it shifts.
So we will choose the smallest k, which is 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `0 <= nums[i] < nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut max_score = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum[(n + i - nums[i] as usize) % n] += 1;
            prefix_sum[i] -= 1;
        }

        for i in (0..n).rev() {
            prefix_sum[i] += prefix_sum[i + 1];
            if max_score <= prefix_sum[i] {
                max_score = prefix_sum[i];
                ret = i;
            }
        }

        ret as i32
    }
}
```
