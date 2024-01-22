# 1438. Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
Given an array of integers `nums` and an integer `limit`, return the size of the longest **non-empty** subarray such that the absolute difference between any two elements of this subarray is less than or equal to `limit`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [8,2,4,7], limit = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> All subarrays are:
[8] with maximum absolute diff |8-8| = 0 <= 4.
[8,2] with maximum absolute diff |8-2| = 6 > 4.
[8,2,4] with maximum absolute diff |8-2| = 6 > 4.
[8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
[2] with maximum absolute diff |2-2| = 0 <= 4.
[2,4] with maximum absolute diff |2-4| = 2 <= 4.
[2,4,7] with maximum absolute diff |2-7| = 5 > 4.
[4] with maximum absolute diff |4-4| = 0 <= 4.
[4,7] with maximum absolute diff |4-7| = 3 <= 4.
[7] with maximum absolute diff |7-7| = 0 <= 4.
Therefore, the size of the longest subarray is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,1,2,4,7,2], limit = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> The subarray [2,4,7,2] is the longest since the maximum absolute diff is |2-7| = 5 <= 5.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [4,2,2,2,4,4,2,2], limit = 0
<strong>Output:</strong> 3
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= limit <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut i = 0;
        let mut ret = 1;

        for j in 0..nums.len() {
            max_heap.push((nums[j], j));
            min_heap.push((-nums[j], j));

            while let Some(&(x, k)) = max_heap.peek() {
                if x - nums[j] > limit {
                    max_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }
            while let Some(&(x, k)) = min_heap.peek() {
                if nums[j] + x > limit {
                    min_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }

            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
