# 674. Longest Continuous Increasing Subsequence
Given an unsorted array of integers, find the length of longest ```continuous``` increasing subsequence (subarray).

#### Example 1:
<pre>
<strong>Input:</strong> [1,3,5,4,7]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest continuous increasing subsequence is [1,3,5], its length is 3. 
Even though [1,3,5,7] is also an increasing subsequence, it's not a continuous one where 5 and 7 are separated by 4. 
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [2,2,2,2,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The longest continuous increasing subsequence is [2], its length is 1.
</pre>

**Note:** Length of the array will not exceed 10,000.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut max_len = 0;

        for j in 1..=nums.len() {
            if j == nums.len() || nums[j] <= nums[j - 1] {
                max_len = max_len.max(j - i);
                i = j;
            }
        }

        max_len as i32
    }
}
```
