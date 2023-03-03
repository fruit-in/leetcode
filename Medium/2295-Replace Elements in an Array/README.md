# 2295. Replace Elements in an Array
You are given a **0-indexed** array `nums` that consists of `n` **distinct** positive integers. Apply `m` operations to this array, where in the <code>i<sup>th</sup></code> operation you replace the number `operations[i][0]` with `operations[i][1]`.

It is guaranteed that in the <code>i<sup>th</sup></code> operation:

* `operations[i][0]` **exists** in `nums`.
* `operations[i][1]` does **not** exist in `nums`.

Return *the array obtained after applying all the operations*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,4,6], operations = [[1,3],[4,7],[6,1]]
<strong>Output:</strong> [3,2,7,1]
<strong>Explanation:</strong> We perform the following operations on nums:
- Replace the number 1 with 3. nums becomes [3,2,4,6].
- Replace the number 4 with 7. nums becomes [3,2,7,6].
- Replace the number 6 with 1. nums becomes [3,2,7,1].
We return the final array [3,2,7,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2], operations = [[1,3],[2,1],[3,2]]
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong> We perform the following operations to nums:
- Replace the number 1 with 3. nums becomes [3,2].
- Replace the number 2 with 1. nums becomes [3,1].
- Replace the number 3 with 2. nums becomes [2,1].
We return the array [2,1].
</pre>

#### Constraints:
* `n == nums.length`
* `m == operations.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* All the values of `nums` are **distinct**.
* `operations[i].length == 2`
* <code>1 <= nums[i], operations[i][0], operations[i][1] <= 10<sup>6</sup></code>
* `operations[i][0]` will exist in `nums` when applying the <code>i<sup>th</sup></code> operation.
* `operations[i][1]` will not exist in `nums` when applying the <code>i<sup>th</sup></code> operation.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut indices = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect::<HashMap<_, _>>();

        for operation in operations {
            let i = indices.remove(&operation[0]).unwrap();
            nums[i] = operation[1];
            indices.insert(nums[i], i);
        }

        nums
    }
}
```
