# 34. Find First and Last Position of Element in Sorted Array
Given an array of integers `nums` sorted in non-decreasing order, find the starting and ending position of a given `target` value.

If `target` is not found in the array, return `[-1, -1]`.

You must write an algorithm with `O(log n)` runtime complexity.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,7,7,8,8,10], target = 8
<strong>Output:</strong> [3,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,7,7,8,8,10], target = 6
<strong>Output:</strong> [-1,-1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [], target = 0
<strong>Output:</strong> [-1,-1]
</pre>

#### Constraints:
* <code>0 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* `nums` is a non-decreasing array.
* <code>-10<sup>9</sup> <= target <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut ret = vec![-1, -1];

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if nums[left] != target {
            return vec![-1, -1];
        }

        ret[0] = left as i32;
        left = 0;
        right = n - 1;

        while left < right {
            let mid = (left + right + 1) / 2;

            if nums[mid] <= target {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        ret[1] = left as i32;

        ret
    }
}
```
