# 448. Find All Numbers Disappeared in an Array
Given an array of integers where 1 ≤ a[i] ≤ *n* (*n* = size of array), some elements appear twice and others appear once.

Find all the elements of [1, *n*] inclusive that do not appear in this array.

Could you do it without extra space and in O(*n*) runtime? You may assume the returned list does not count as extra space.

#### Example:
<pre>
<strong>Input:</strong>
[4,3,2,7,8,2,3,1]
<strong>Output:</strong>
[5,6]
</pre>

## Solutions (Rust)

### 1. Swap Positions
```Rust
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let mut curr = nums[i];

            while nums[curr as usize - 1] != curr {
                let next = nums[curr as usize - 1];
                nums[curr as usize - 1] = curr;
                curr = next;
            }
        }

        for i in 0..nums.len() {
            if i + 1 != nums[i] as usize {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```

### 2. Mark Positions
```Rust
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j].abs();
        }

        for i in 0..nums.len() {
            if nums[i] > 0 {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```
