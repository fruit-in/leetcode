# 1296. Divide Array in Sets of K Consecutive Numbers
Given an array of integers `nums` and a positive integer `k`, find whether it's possible to divide this array into sets of `k` consecutive numbers

Return `True` if it is possible. Otherwise, return `False`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,3,4,4,5,6], k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> Array can be divided into [1,2,3,4] and [3,4,5,6].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
<strong>Output:</strong> true
<strong>Explanation:</strong> Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,3,2,2,1,1], k = 3
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> Each array should be divided in subarrays of size 3.
</pre>

#### Constraints:
* <code>1 <= k <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

**Note:** This question is the same as 846: https://leetcode.com/problems/hand-of-straights/

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        let mut needs: HashMap<i32, Vec<i32>> = HashMap::new();
        nums.sort_unstable();

        for x in nums {
            if let Some(v) = needs.get_mut(&(x - 1)) {
                match v.pop() {
                    Some(1) => (),
                    Some(y) => needs.entry(x).or_insert(vec![]).push(y - 1),
                    None => needs.entry(x).or_insert(vec![]).push(k - 1),
                }
            } else if k > 1 {
                needs.entry(x).or_insert(vec![]).push(k - 1);
            }
        }

        needs.values().all(|v| v.is_empty())
    }
}
```
