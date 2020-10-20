# 347. Top K Frequent Elements
Given a non-empty array of integers, return the ***k*** most frequent elements.

#### Example 1:
<pre>
<b>Input:</b> nums = [1,1,1,2,2,3], k = 2
<b>Output:</b> [1,2]
</pre>

#### Example 2:
<pre>
<b>Input:</b> nums = [1], k = 1
<b>Output:</b> [1]
</pre>

#### Note:
* You may assume *k* is always valid, 1 ≤ *k* ≤ number of unique elements.
* Your algorithm's time complexity **must be** better than O(*n* log *n*), where *n* is the array's size.
* It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
* You can return the answer in any order.

## Solutions (Rust)

### 1. Sort
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut counter = counter.iter().collect::<Vec<_>>();

        counter.sort_unstable_by_key(|c| c.1);

        counter
            .iter()
            .rev()
            .take(k as usize)
            .map(|(k, v)| **k)
            .collect()
    }
}
```
