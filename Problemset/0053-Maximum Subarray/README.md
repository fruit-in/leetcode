# 53. Maximum Subarray
Given an integer array ```nums```, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

#### Example:
<pre>
<strong>Input:</strong> [-2,1,-3,4,-1,2,1,-5,4],
<strong>Output:</strong> 6
<strong>Explanation:</strong> [4,-1,2,1] has the largest sum = 6.
</pre>

#### Follow up:
If you have figured out the O(*n*) solution, try coding another solution using the divide and conquer approach, which is more subtle.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum > max {
                    max = sum;
                }
            }
        }
        max
    }
}
```

### 2. One Pass
```Rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = std::i32::MIN;
        let mut min_sum = 0;

        for n in nums {
            min_sum = min_sum.min(sum);
            sum += n;
            max_sum = max_sum.max(sum - min_sum);
        }
        max_sum
    }
}
```
