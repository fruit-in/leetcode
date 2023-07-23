# 2016. Maximum Difference Between Increasing Elements
Given a **0-indexed** integer array `nums` of size `n`, find the **maximum difference** between `nums[i]` and `nums[j]` (i.e., `nums[j] - nums[i]`), such that `0 <= i < j < n` and `nums[i] < nums[j]`.

Return *the **maximum difference***. If no such `i` and `j` exists, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [7,1,5,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [9,4,3,2]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
There is no i and j such that i < j and nums[i] < nums[j].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,5,2,10]
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The maximum difference occurs with i = 0 and j = 3, nums[j] - nums[i] = 10 - 1 = 9.
</pre>

#### Constraints:
* `n == nums.length`
* `2 <= n <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        let mut ret = -1;

        for &num in &nums[1..] {
            if num < min {
                min = num;
                max = num;
            } else if num > max {
                max = num;
                ret = ret.max(max - min);
            }
        }

        ret
    }
}
```
