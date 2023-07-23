# 2190. Most Frequent Number Following Key In an Array
You are given a **0-indexed** integer array `nums`. You are also given an integer `key`, which is present in `nums`.

For every unique integer `target` in `nums`, **count** the number of times `target` immediately follows an occurrence of `key` in `nums`. In other words, count the number of indices `i` such that:
* `0 <= i <= nums.length - 2`,
* `nums[i] == key` and,
* `nums[i + 1] == target`.

Return *the* `target` *with the **maximum** count*. The test cases will be generated such that the `target` with maximum count is unique.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,100,200,1,100], key = 1
<strong>Output:</strong> 100
<strong>Explanation:</strong> For target = 100, there are 2 occurrences at indices 1 and 4 which follow an occurrence of key.
No other integers follow an occurrence of key, so we return 100.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2,2,3], key = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> For target = 2, there are 3 occurrences at indices 1, 2, and 3 which follow an occurrence of key.
For target = 3, there is only one occurrence at index 4 which follows an occurrence of key.
target = 2 has the maximum number of occurrences following an occurrence of key, so we return 2.
</pre>

#### Constraints:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`
* The test cases will be generated such that the answer is unique.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = HashMap::new();
        let mut max = 0;
        let mut ret = 0;

        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                *count.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }

        count.into_iter().map(|(k, v)| (v, k)).max().unwrap().1
    }
}
```
