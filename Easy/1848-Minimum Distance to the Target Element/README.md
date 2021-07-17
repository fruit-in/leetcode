# 1848. Minimum Distance to the Target Element
Given an integer array `nums` (**0-indexed**) and two integers `target` and `start`, find an index `i` such that `nums[i] == target` and `abs(i - start)` is **minimized**. Note that `abs(x)` is the absolute value of `x`.

Return `abs(i - start)`.

It is **guaranteed** that `target` exists in `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5], target = 5, start = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> nums[4] = 5 is the only value equal to target, so the answer is abs(4 - 3) = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1], target = 1, start = 0
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums[0] = 1 is the only value equal to target, so the answer is abs(0 - 0) = 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1,1,1,1,1,1], target = 1, start = 0
<strong>Output:</strong> 0
<strong>Explanation:</strong> Every value of nums is 1, but nums[0] minimizes abs(i - start), which is abs(0 - 0) = 0.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `0 <= start < nums.length`
* `target` is in `nums`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;

        for i in 0..nums.len() {
            if nums[(start + i).min(nums.len() - 1)] == target
                || nums[start.saturating_sub(i)] == target
            {
                return i as i32;
            }
        }

        unreachable!()
    }
}
```
