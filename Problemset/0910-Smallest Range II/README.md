# 910. Smallest Range II
You are given an integer array `nums` and an integer `k`.

For each index `i` where `0 <= i < nums.length`, change `nums[i]` to be either `nums[i] + k` or `nums[i] - k`.

The **score** of `nums` is the difference between the maximum and minimum elements in `nums`.

Return *the minimum **score** of* `nums` *after changing the values at each index*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1], k = 0
<strong>Output:</strong> 0
<strong>Explanation:</strong> The score is max(nums) - min(nums) = 1 - 1 = 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,10], k = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,3,6], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> Change nums to be [4, 6, 3]. The score is max(nums) - min(nums) = 6 - 3 = 3.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>4</sup></code>
* <code>0 <= k <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums.into_iter().map(|x| x + k).collect::<Vec<_>>();
        let mut ret = 0;

        nums.sort_unstable();

        if nums[n - 1] - 2 * k < nums[0] {
            ret = nums[n - 1] - nums[0];
        } else {
            for i in 1..nums.len() {
                if nums[i] - 2 * k >= nums[0] {
                    ret = nums[i - 1].max(nums[n - 1] - 2 * k) - nums[0];
                    break;
                }
            }
        }

        for i in 1..nums.len() {
            if nums[i] - 2 * k > nums[0] {
                break;
            }

            ret = ret.min(nums[i - 1].max(nums[n - 1] - 2 * k) - nums[i] + 2 * k);
        }

        ret
    }
}
```
