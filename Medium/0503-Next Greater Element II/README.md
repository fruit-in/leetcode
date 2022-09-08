# 503. Next Greater Element II
Given a circular integer array `nums` (i.e., the next element of `nums[nums.length - 1]` is `nums[0]`), return *the **next greater number** for every element in* `nums`.

The **next greater number** of a number `x` is the first greater number to its traversing-order next in the array, which means you could search circularly to find its next greater number. If it doesn't exist, return `-1` for this number.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,1]
<strong>Output:</strong> [2,-1,2]
<strong>Explanation:</strong> The first 1's next greater number is 2;
The number 2 can't find next greater number.
The second 1's next greater number needs to search circularly, which is also 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,3]
<strong>Output:</strong> [2,3,4,-1,4]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![-1; nums.len()];

        for i in (0..nums.len()).chain(0..nums.len()) {
            while !stack.is_empty() && nums[i] > nums[*stack.last().unwrap()] {
                ret[stack.pop().unwrap()] = nums[i];
            }

            if ret[i] == -1 {
                stack.push(i);
            }
        }

        ret
    }
}
```
