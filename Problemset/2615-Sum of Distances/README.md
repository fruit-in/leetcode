# 2615. Sum of Distances
You are given a **0-indexed** integer array `nums`. There exists an array `arr` of length `nums.length`, where `arr[i]` is the sum of `|i - j|` over all `j` such that `nums[j] == nums[i]` and `j != i`. If there is no such `j`, set `arr[i]` to be `0`.

Return *the array* `arr`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,1,1,2]
<strong>Output:</strong> [5,0,3,4,0]
<strong>Explanation:</strong>
When i = 0, nums[0] == nums[2] and nums[0] == nums[3]. Therefore, arr[0] = |0 - 2| + |0 - 3| = 5.
When i = 1, arr[1] = 0 because there is no other index with value 3.
When i = 2, nums[2] == nums[0] and nums[2] == nums[3]. Therefore, arr[2] = |2 - 0| + |2 - 3| = 3.
When i = 3, nums[3] == nums[0] and nums[3] == nums[2]. Therefore, arr[3] = |3 - 0| + |3 - 2| = 4.
When i = 4, arr[4] = 0 because there is no other index with value 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,5,3]
<strong>Output:</strong> [0,0,0]
<strong>Explanation:</strong> Since each element in nums is distinct, arr[i] = 0 for all i.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

**Note:** This question is the same as [2121: Intervals Between Identical Elements](https://leetcode.com/problems/intervals-between-identical-elements/description/).

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut count = HashMap::new();
        let mut indices_sum = HashMap::new();
        let mut arr = vec![0; nums.len()];

        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] = count[&nums[i]] * i as i64 - indices_sum[&nums[i]];
        }

        count.clear();
        indices_sum.clear();

        for i in (0..nums.len()).rev() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] += indices_sum[&nums[i]] - count[&nums[i]] * i as i64;
        }

        arr
    }
}
```
