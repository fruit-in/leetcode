# 154. Find Minimum in Rotated Sorted Array II
Suppose an array of length `n` sorted in ascending order is **rotated** between `1` and `n` times. For example, the array `nums = [0,1,4,4,5,6,7]` might become:
* `[4,5,6,7,0,1,4]` if it was rotated `4` times.
* `[0,1,4,4,5,6,7]` if it was rotated `7` times.

Notice that **rotating** an array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time results in the array `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.

Given the sorted rotated array `nums` that may contain **duplicates**, return *the minimum element of this array*.

You must decrease the overall operation steps as much as possible.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,5]
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2,0,1]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `n == nums.length`
* `1 <= n <= 5000`
* `-5000 <= nums[i] <= 5000`
* `nums` is sorted and rotated between `1` and `n` times.

**Follow up:** This problem is similar to [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/), but `nums` may contain **duplicates**. Would this affect the runtime complexity? How and why?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut stack = vec![(0, nums.len() - 1)];
        let mut ret = nums[0];

        while let Some((l, r)) = stack.pop() {
            if l == r || nums[l] < nums[r] {
                ret = ret.min(nums[l]);
                continue;
            }

            let mid = (l + r) / 2;

            if nums[mid] > nums[l] || (nums[l] != nums[r] && nums[mid] == nums[l]) {
                stack.push((mid + 1, r));
            } else if nums[mid] < nums[r] || (nums[l] != nums[r] && nums[mid] == nums[r]) {
                stack.push((l, mid));
            } else {
                stack.push((mid + 1, r));
                stack.push((l, mid));
            }
        }

        ret
    }
}
```
