# 16. 3Sum Closest
Given an array `nums` of *n* integers and an integer `target`, find three integers in `nums` such that the sum is closest to `target`. Return the sum of the three integers. You may assume that each input would have exactly one solution.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-1,2,1,-4], target = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
</pre>

#### Constraints:
* `3 <= nums.length <= 10^3`
* `-10^3 <= nums[i] <= 10^3`
* `-10^4 <= target <= 10^4`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut diff = std::i32::MAX;
        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if (target - sum).abs() < diff.abs() {
                    diff = target - sum;
                }
                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        target - diff
    }
}
```
