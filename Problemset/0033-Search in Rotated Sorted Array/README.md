# 33. Search in Rotated Sorted Array
Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.

(i.e., `[0,1,2,4,5,6,7]` might become `[4,5,6,7,0,1,2]`).

You are given a target value to search. If found in the array return its index, otherwise return `-1`.

You may assume no duplicate exists in the array.

Your algorithm's runtime complexity must be in the order of *O*(log *n*).

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,5,6,7,0,1,2], target = 0
<strong>Output:</strong> 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,5,6,7,0,1,2], target = 3
<strong>Output:</strong> -1
</pre>

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return m as i32;
            }
            if nums[l] <= nums[m] && nums[m] <= nums[r - 1] {
                if target < nums[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[l] >= nums[m] {
                if target < nums[m] || target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[m] >= nums[r - 1] {
                if target < nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }

        -1
    }
}
```
