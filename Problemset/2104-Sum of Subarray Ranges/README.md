# 2104. Sum of Subarray Ranges
You are given an integer array `nums`. The **range** of a subarray of `nums` is the difference between the largest and smallest element in the subarray.

Return *the **sum of all** subarray ranges of* `nums`.

A subarray is a contiguous **non-empty** sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 6 subarrays of nums are the following:
[1], range = largest - smallest = 1 - 1 = 0
[2], range = 2 - 2 = 0
[3], range = 3 - 3 = 0
[1,2], range = 2 - 1 = 1
[2,3], range = 3 - 2 = 1
[1,2,3], range = 3 - 1 = 2
So the sum of all ranges is 0 + 0 + 0 + 1 + 1 + 2 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,3,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 6 subarrays of nums are the following:
[1], range = largest - smallest = 1 - 1 = 0
[3], range = 3 - 3 = 0
[3], range = 3 - 3 = 0
[1,3], range = 3 - 1 = 2
[3,3], range = 3 - 3 = 0
[1,3,3], range = 3 - 1 = 2
So the sum of all ranges is 0 + 0 + 0 + 2 + 0 + 2 = 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [4,-2,-3,4,1]
<strong>Output:</strong> 59
<strong>Explanation:</strong> The sum of all subarray ranges of nums is 59.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

**Follow-up:** Could you find a solution with `O(n)` time complexity?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count_as_max = vec![0; n];
        let mut count_as_min = vec![0; n];

        let mut dec_stack = vec![];
        let mut inc_stack = vec![];
        for i in 0..n {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 < nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 > nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] = i as i64 - dec_stack.last().unwrap_or(&(-1, 0)).0;
            count_as_min[i] = i as i64 - inc_stack.last().unwrap_or(&(-1, 0)).0;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        dec_stack = vec![];
        inc_stack = vec![];
        for i in (0..n).rev() {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 <= nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 >= nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] *= dec_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            count_as_min[i] *= inc_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        (0..n)
            .map(|i| (count_as_max[i] - count_as_min[i]) * nums[i] as i64)
            .sum()
    }
}
```
