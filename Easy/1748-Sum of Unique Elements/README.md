# 1748. Sum of Unique Elements
You are given an integer array `nums`. The unique elements of an array are the elements that appear **exactly once** in the array.

Return *the **sum** of all the unique elements of* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The unique elements are [1,3], and the sum is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1,1,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no unique elements, and the sum is 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 15
<strong>Explanation:</strong> The unique elements are [1,2,3,4,5], and the sum is 15.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];

        for num in nums {
            count[num as usize] += 1;
        }

        count
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 1)
            .map(|(n, _)| n as i32)
            .sum()
    }
}
```
