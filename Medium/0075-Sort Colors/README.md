# 75. Sort Colors
Given an array with *n* objects colored red, white or blue, sort them **in-place** so that objects of the same color are adjacent, with the colors in the order red, white and blue.

Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.

**Note:** You are not suppose to use the library's sort function for this problem.

#### Example:
<pre>
<strong>Input:</strong> [2,0,2,1,1,0]
<strong>Output:</strong> [0,0,1,1,2,2]
</pre>

#### Follow up:
* A rather straight forward solution is a two-pass algorithm using counting sort.<br>
First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
* Could you come up with a one-pass algorithm using only constant space?

## Solutions (Rust)

### 1. One Pass
```Rust
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0usize;
        let mut right = nums.len() - 1;
        let mut index = 0usize;
        while index <= right {
            if nums[index] == 0 && index > left {
                while left < index && nums[left] == 0 {
                    left += 1;
                }
                nums.swap(left, index);
            } else if nums[index] == 2 && index < right {
                while right > index && nums[right] == 2 {
                    right -= 1;
                }
                nums.swap(index, right);
            } else {
                index += 1;
            }
        }
    }
}
```
