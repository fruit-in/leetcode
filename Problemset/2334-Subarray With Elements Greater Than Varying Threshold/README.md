# 2334. Subarray With Elements Greater Than Varying Threshold
You are given an integer array `nums` and an integer `threshold`.

Find any subarray of `nums` of length `k` such that **every** element in the subarray is **greater** than `threshold / k`.

Return *the **size** of **any** such subarray*. If there is no such subarray, return `-1`.

A **subarray** is a contiguous non-empty sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,4,3,1], threshold = 6
<strong>Output:</strong> 3
<strong>Explanation:</strong> The subarray [3,4,3] has a size of 3, and every element is greater than 6 / 3 = 2.
Note that this is the only valid subarray.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [6,5,6,5,8], threshold = 7
<strong>Output:</strong> 1
<strong>Explanation:</strong> The subarray [8] has a size of 1, and 8 > 7 / 1 = 7. So 1 is returned.
Note that the subarray [6,5] has a size of 2, and every element is greater than 7 / 2 = 3.5.
Similarly, the subarrays [6,5,6], [6,5,6,5], [6,5,6,5,8] also satisfy the given conditions.
Therefore, 2, 3, 4, or 5 may also be returned.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], threshold <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut sublens = vec![0; nums.len()];
        let mut asc_stack = vec![];

        for i in 0..nums.len() {
            while asc_stack.last().unwrap_or(&(-1, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] = i as i32 - asc_stack.last().unwrap_or(&(-1, 0)).0;
            asc_stack.push((i as i32, nums[i]));
        }

        asc_stack.clear();

        for i in (0..nums.len()).rev() {
            while asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] += asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).0 - i as i32 - 1;
            asc_stack.push((i as i32, nums[i]));

            if nums[i] > threshold / sublens[i] {
                return sublens[i];
            }
        }

        -1
    }
}
```
