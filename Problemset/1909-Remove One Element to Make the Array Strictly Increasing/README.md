# 1909. Remove One Element to Make the Array Strictly Increasing
Given a **0-indexed** integer array `nums`, return `true` *if it can be made **strictly increasing** after removing **exactly one** element, or* `false` *otherwise. If the array is already strictly increasing, return* `true`.

The array `nums` is **strictly increasing** if `nums[i - 1] < nums[i]` for each index `(1 <= i < nums.length)`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,10,5,7]
<strong>Output:</strong> true
<strong>Explanation:</strong> By removing 10 at index 2 from nums, it becomes [1,2,5,7].
[1,2,5,7] is strictly increasing, so return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,1,2]
<strong>Output:</strong> false
<strong>Explanation:</strong> [3,1,2] is the result of removing the element at index 0.
[2,1,2] is the result of removing the element at index 1.
[2,3,2] is the result of removing the element at index 2.
[2,3,1] is the result of removing the element at index 3.
No resulting array is strictly increasing, so return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> false
<strong>Explanation:</strong> The result of removing any element is [1,1].
[1,1] is not strictly increasing, so return false.
</pre>

#### Constraints:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let mut removed = false;

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                if removed {
                    return false;
                }
                if i > 1 && nums[i] <= nums[i - 2] {
                    nums[i] = nums[i - 1];
                }
                removed = true;
            }
        }

        true
    }
}
```
