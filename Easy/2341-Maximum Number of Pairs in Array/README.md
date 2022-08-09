# 2341. Maximum Number of Pairs in Array
You are given a **0-indexed** integer array `nums`. In one operation, you may do the following:
* Choose **two** integers in `nums` that are **equal**.
* Remove both integers from `nums`, forming a **pair**.

The operation is done on `nums` as many times as possible.

Return *a **0-indexed** integer array* `answer` *of size* `2` *where* `answer[0]` *is the number of pairs that are formed and* `answer[1]` *is the number of leftover integers in* `nums` *after doing the operation as many times as possible*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,2,1,3,2,2]
<strong>Output:</strong> [3,1]
<strong>Explanation:</strong>
Form a pair with nums[0] and nums[3] and remove them from nums. Now, nums = [3,2,3,2,2].
Form a pair with nums[0] and nums[2] and remove them from nums. Now, nums = [2,2,2].
Form a pair with nums[0] and nums[1] and remove them from nums. Now, nums = [2].
No more pairs can be formed. A total of 3 pairs have been formed, and there is 1 number leftover in nums.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1]
<strong>Output:</strong> [1,0]
<strong>Explanation:</strong> Form a pair with nums[0] and nums[1] and remove them from nums. Now, nums = [].
No more pairs can be formed. A total of 1 pair has been formed, and there are 0 numbers leftover in nums.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> No pairs can be formed, and there is 1 number leftover in nums.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut answer = vec![0, 0];

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        answer[0] = count.values().map(|&x| x / 2).sum();
        answer[1] = count.values().map(|&x| x % 2).sum();

        answer
    }
}
```
