# 2012. Sum of Beauty in the Array
You are given a **0-indexed** integer array `nums`. For each index `i` (`1 <= i <= nums.length - 2`) the **beauty** of `nums[i]` equals:

* `2`, if `nums[j] < nums[i] < nums[k]`, for **all** `0 <= j < i` and for **all** `i < k <= nums.length - 1`.
* `1`, if `nums[i - 1] < nums[i] < nums[i + 1]`, and the previous condition is not satisfied.
* `0`, if none of the previous conditions holds.

Return *the **sum of beauty** of all* `nums[i]` *where* `1 <= i <= nums.length - 2`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> For each index i in the range 1 <= i <= 1:
- The beauty of nums[1] equals 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,4,6,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong> For each index i in the range 1 <= i <= 2:
- The beauty of nums[1] equals 1.
- The beauty of nums[2] equals 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,2,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> For each index i in the range 1 <= i <= 1:
- The beauty of nums[1] equals 0.
</pre>

#### Constraints:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max = vec![i32::MIN; nums.len()];
        let mut min = vec![i32::MAX; nums.len()];
        let mut ret = 0;

        for i in 1..nums.len() {
            max[i] = max[i - 1].max(nums[i - 1]);
            min[n - 1 - i] = min[n - i].min(nums[n - i]);
        }

        for i in 1..nums.len() - 1 {
            if max[i] < nums[i] && nums[i] < min[i] {
                ret += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                ret += 1;
            }
        }

        ret
    }
}
```
