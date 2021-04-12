# 81. Search in Rotated Sorted Array II
There is an integer array `nums` sorted in non-decreasing order (not necessarily with **distinct** values).

Before being passed to your function, `nums` is **rotated** at an unknown pivot index `k` (`0 <= k < nums.length`) such that the resulting array is `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]` (**0-indexed**). For example, `[0,1,2,4,4,4,5,6,6,7]` might be rotated at pivot index `5` and become `[4,5,6,6,7,0,1,2,4,4]`.

Given the array `nums` **after** the rotation and an integer `target`, return `true` *if* `target` *is in* `nums`*, or* `false` *if it is not in* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,5,6,0,0,1,2], target = 0
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,5,6,0,0,1,2], target = 3
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= nums.length <= 5000`
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>
* `nums` is guaranteed to be rotated at some pivot.
* <code>-10<sup>4</sup> <= target <= 10<sup>4</sup></code>

**Follow up:** This problem is the same as [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/description/), where `nums` may contain **duplicates**. Would this affect the runtime complexity? How and why?

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return true;
            }
            if nums[l] == nums[m] && nums[m] == nums[r - 1] {
                l += 1;
                r -= 1;
            } else if nums[l] <= nums[m] {
                if target < nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                if target < nums[m] || target > *nums.last().unwrap() {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }

        false
    }
}
```
