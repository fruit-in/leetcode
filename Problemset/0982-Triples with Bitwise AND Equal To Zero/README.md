# 982. Triples with Bitwise AND Equal To Zero
Given an integer array nums, return *the number of **AND triples***.

An **AND triple** is a triple of indices `(i, j, k)` such that:

* `0 <= i < nums.length`
* `0 <= j < nums.length`
* `0 <= k < nums.length`
* `nums[i] & nums[j] & nums[k] == 0`, where `&` represents the bitwise-AND operator.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,3]
<strong>Output:</strong> 12
<strong>Explanation:</strong> We could choose the following i, j, k triples:
(i=0, j=0, k=1) : 2 & 2 & 1
(i=0, j=1, k=0) : 2 & 1 & 2
(i=0, j=1, k=1) : 2 & 1 & 1
(i=0, j=1, k=2) : 2 & 1 & 3
(i=0, j=2, k=1) : 2 & 3 & 1
(i=1, j=0, k=0) : 1 & 2 & 2
(i=1, j=0, k=1) : 1 & 2 & 1
(i=1, j=0, k=2) : 1 & 2 & 3
(i=1, j=1, k=0) : 1 & 1 & 2
(i=1, j=2, k=0) : 1 & 3 & 2
(i=2, j=0, k=1) : 3 & 2 & 1
(i=2, j=1, k=0) : 3 & 1 & 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0,0]
<strong>Output:</strong> 27
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>0 <= nums[i] < 2<sup>16</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                *count.entry(nums[i] & nums[j]).or_insert(0) += 1;
            }
        }

        for k in 0..nums.len() {
            for (x, c) in count.iter() {
                if x & nums[k] == 0 {
                    ret += c;
                }
            }
        }

        ret
    }
}
```
