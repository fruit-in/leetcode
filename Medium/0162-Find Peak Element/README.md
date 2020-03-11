# 162. Find Peak Element
A peak element is an element that is greater than its neighbors.

Given an input array ```nums```, where ```nums[i] ≠ nums[i+1]```, find a peak element and return its index.

The array may contain multiple peaks, in that case return the index to any one of the peaks is fine.

You may imagine that ```nums[-1] = nums[n] = -∞```.

#### Example 1:
<pre>
<strong>Input:</strong> <strong>nums</strong> = [1,2,3,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 3 is a peak element and your function should return the index number 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> <strong>nums</strong> = [1,2,1,3,5,6,4]
<strong>Output:</strong> 1 or 5
<strong>Explanation:</strong> Your function can return either index number 1 where the peak element is 2, 
             or index number 5 where the peak element is 6.
</pre>

#### Note:
Your solution should be in logarithmic complexity.

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[m + 1] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32
    }
}
```
