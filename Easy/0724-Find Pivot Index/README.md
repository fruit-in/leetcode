# 724. Find Pivot Index
Given an array of integers <code>nums</code>, write a method that returns the "pivot" index of this array.

We define the pivot index as the index where the sum of the numbers to the left of the index is equal to the sum of the numbers to the right of the index.

If no such index exists, we should return -1. If there are multiple pivot indexes, you should return the left-most pivot index.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1, 7, 3, 6, 5, 6]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The sum of the numbers to the left of index 3 (nums[3] = 6) is equal to the sum of numbers to the right of index 3.
Also, 3 is the first index where this occurs.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1, 2, 3]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
There is no index that satisfies the conditions in the problem statement.
</pre>

#### Note:
* The length of <code>nums</code> will be in the range <code>[0, 10000]</code>.
* Each element <code>nums[i]</code> will be an integer in the range <code>[-1000, 1000]</code>.

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for i in 0..nums.len() {
            if 2 * left_sum == total_sum - nums[i] {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
```
