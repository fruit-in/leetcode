# 442. Find All Duplicates in an Array
Given an array of integers, 1 ≤ a[i] ≤ *n* (*n* = size of array), some elements appear **twice** and others appear **once**.

Find all the elements that appear **twice** in this array.

Could you do it without extra space and in O(*n*) runtime?

#### Example:
<pre>
<strong>Input:</strong>
[4,3,2,7,8,2,3,1]
<strong>Output:</strong>
[2,3]
</pre>

## Solutions (Rust)

### 1. Mark Positions
```Rust
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j];
            if nums[j] > 0 {
                ret.push(j as i32 + 1);
            }
        }

        ret
    }
}
```
