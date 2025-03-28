# 795. Number of Subarrays with Bounded Maximum
Given an integer array `nums` and two integers `left` and `right`, return *the number of contiguous non-empty **subarrays** such that the value of the maximum array element in that subarray is in the range* `[left, right]`.

The test cases are generated so that the answer will fit in a **32-bit** integer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,4,3], left = 2, right = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three subarrays that meet the requirements: [2], [2, 1], [3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,9,2,5,6], left = 2, right = 8
<strong>Output:</strong> 7
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= left <= right <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut desc_stack = vec![];
        let mut countl = vec![0; nums.len()];
        let mut countr = vec![0; nums.len()];

        for i in 0..nums.len() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] < nums[i] {
                desc_stack.pop();
            }

            countl[i] = i as i32 - *desc_stack.last().unwrap_or(&-1) - 1;
            desc_stack.push(i as i32);
        }
        desc_stack.clear();
        for i in (0..nums.len()).rev() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] <= nums[i] {
                desc_stack.pop();
            }

            countr[i] = *desc_stack.last().unwrap_or(&(nums.len() as i32)) - i as i32 - 1;
            desc_stack.push(i as i32);
        }

        (0..nums.len())
            .filter(|&i| nums[i] >= left && nums[i] <= right)
            .map(|i| (countl[i] + 1) * (countr[i] + 1))
            .sum()
    }
}
```
