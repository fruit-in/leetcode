# 659. Split Array into Consecutive Subsequences
You are given an integer array `nums` that is **sorted in non-decreasing order**.

Determine if it is possible to split `nums` into **one or more subsequences** such that **both** of the following conditions are true:

* Each subsequence is a **consecutive increasing sequence** (i.e. each integer is **exactly one** more than the previous integer).
* All subsequences have a length of `3` **or more**.

Return `true` *if you can split* `nums` *according to the above conditions, or* `false` *otherwise*.

A **subsequence** of an array is a new array that is formed from the original array by deleting some (can be none) of the elements without disturbing the relative positions of the remaining elements. (i.e., `[1,3,5]` is a subsequence of `[1,2,3,4,5]` while `[1,3,2]` is not).

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,3,4,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> nums can be split into the following subsequences:
[1,2,3,3,4,5] --> 1, 2, 3
[1,2,3,3,4,5] --> 3, 4, 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,3,4,4,5,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> nums can be split into the following subsequences:
[1,2,3,3,4,4,5,5] --> 1, 2, 3, 4, 5
[1,2,3,3,4,4,5,5] --> 3, 4, 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,4,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to split nums into consecutive increasing subsequences of length 3 or more.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `-1000 <= nums[i] <= 1000`
* `nums` is sorted in **non-decreasing** order.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for &num in &nums {
            let (a, b, c) = *count.get(&(num - 1)).unwrap_or(&(0, 0, 0));
            let (d, e, f) = *count.get(&num).unwrap_or(&(0, 0, 0));

            if a > 0 {
                count.insert(num - 1, (a - 1, b, c));
                count.insert(num, (d, e + 1, f));
            } else if b > 0 {
                count.insert(num - 1, (a, b - 1, c));
                count.insert(num, (d, e, f + 1));
            } else if c > 0 {
                count.insert(num - 1, (a, b, c - 1));
                count.insert(num, (d, e, f + 1));
            } else {
                count.insert(num, (d + 1, e, f));
            }
        }

        count.values().all(|&(a, b, _)| a | b == 0)
    }
}
```
