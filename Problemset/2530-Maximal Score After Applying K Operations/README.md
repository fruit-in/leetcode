# 2530. Maximal Score After Applying K Operations
You are given a **0-indexed** integer array `nums` and an integer `k`. You have a **starting score** of `0`.

In one **operation**:
1. choose an index `i` such that `0 <= i < nums.length`,
2. increase your **score** by `nums[i]`, and
3. replace `nums[i]` with `ceil(nums[i] / 3)`.

Return *the maximum possible **score** you can attain after applying **exactly*** `k` *operations*.

The ceiling function `ceil(val)` is the least integer greater than or equal to `val`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,10,10,10,10], k = 5
<strong>Output:</strong> 50
<strong>Explanation:</strong> Apply the operation to each array element exactly once. The final score is 10 + 10 + 10 + 10 + 10 = 50.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,10,3,3,3], k = 3
<strong>Output:</strong> 17
<strong>Explanation:</strong> You can do the following operations:
Operation 1: Select i = 1, so nums becomes [1,4,3,3,3]. Your score increases by 10.
Operation 2: Select i = 1, so nums becomes [1,2,3,3,3]. Your score increases by 4.
Operation 3: Select i = 2, so nums becomes [1,2,1,3,3]. Your score increases by 3.
The final score is 10 + 4 + 3 = 17.
</pre>

#### Constraints:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<BinaryHeap<_>>();
        let mut ret = 0;

        for _ in 0..k {
            let x = nums.pop().unwrap();
            nums.push((x + 2) / 3);
            ret += x;
        }

        ret
    }
}
```
