# 1218. Longest Arithmetic Subsequence of Given Difference
Given an integer array `arr` and an integer `difference`, return the length of the longest subsequence in `arr` which is an arithmetic sequence such that the difference between adjacent elements in the subsequence equals `difference`.

A **subsequence** is a sequence that can be derived from `arr` by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,4], difference = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest arithmetic subsequence is [1,2,3,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,3,5,7], difference = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The longest arithmetic subsequence is any single element.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,5,7,8,5,3,4,2,1], difference = -2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest arithmetic subsequence is [7,5,3,1].
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= arr[i], difference <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut longest_length = HashMap::new();

        for &num in &arr {
            longest_length.insert(
                num,
                *longest_length.get(&(num - difference)).unwrap_or(&0) + 1,
            );
        }

        *longest_length.values().max().unwrap()
    }
}
```
