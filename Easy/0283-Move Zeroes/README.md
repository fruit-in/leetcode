# 283. Move Zeroes
Given an array <code>nums</code>, write a function to move all <code>0</code>'s to the end of it while maintaining the relative order of the non-zero elements.

#### Example:
<pre>
<strong>Input:</strong> [0,1,0,3,12]
<strong>Output:</strong> [1,3,12,0,0]
</pre>

#### Note:
1. You must do this **in-place** without making a copy of the array.
2. Minimize the total number of operations.

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}
```

### 2. One Pointer
```Rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for _ in 0..nums.len() {
            if nums[i] == 0 {
                nums.remove(i);
                nums.push(0);
            } else {
                i += 1;
            }
        }
    }
}
```
