# 1696. Jump Game VI
You are given a **0-indexed** integer array `nums` and an integer `k`.

You are initially standing at index `0`. In one move, you can jump at most `k` steps forward without going outside the boundaries of the array. That is, you can jump from index `i` to any index in the range `[i + 1, min(n - 1, i + k)]` **inclusive**.

You want to reach the last index of the array (index `n - 1`). Your **score** is the **sum** of all `nums[j]` for each index `j` you visited in the array.

Return *the **maximum score** you can get*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,-1,-2,4,-7,3], k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> You can choose your jumps forming the subsequence [1,-1,4,3] (underlined above). The sum is 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,-5,-2,4,0,3], k = 3
<strong>Output:</strong> 17
<strong>Explanation:</strong> You can choose your jumps forming the subsequence [10,4,3] (underlined above). The sum is 17.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,-5,-20,4,-1,3,-6,-3], k = 2
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::from([(0, nums[0])]);

        for i in 1..nums.len() {
            if i - deque[0].0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque[0].1;

            while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                deque.pop_back();
            }

            deque.push_back((i, x));
        }

        deque.pop_back().unwrap().1
    }
}
```
