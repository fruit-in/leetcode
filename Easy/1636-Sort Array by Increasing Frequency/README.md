# 1636. Sort Array by Increasing Frequency
Given an array of integers `nums`, sort the array in **increasing** order based on the frequency of the values. If multiple values have the same frequency, sort them in **decreasing** order.

Return the *sorted array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,2,2,2,3]
<strong>Output:</strong> [3,1,1,2,2,2]
<strong>Explanation:</strong> '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,1,3,2]
<strong>Output:</strong> [1,3,3,2,2]
<strong>Explanation:</strong> '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [-1,1,-6,4,5,-6,1,4,1]
<strong>Output:</strong> [5,-1,4,4,-6,-6,1,1,1]
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `-100 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Sort
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.sort_by_key(|k| count.get(k).unwrap());

        nums
    }
}
```
