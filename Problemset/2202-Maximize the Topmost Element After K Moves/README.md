# 2202. Maximize the Topmost Element After K Moves
You are given a **0-indexed** integer array `nums` representing the contents of a **pile**, where `nums[0]` is the topmost element of the pile.

In one move, you can perform **either** of the following:

* If the pile is not empty, **remove** the topmost element of the pile.
* If there are one or more removed elements, **add** any one of them back onto the pile. This element becomes the new topmost element.

You are also given an integer `k`, which denotes the total number of moves to be made.

Return *the **maximum value** of the topmost element of the pile possible after **exactly*** `k` *moves*. In case it is not possible to obtain a non-empty pile after `k` moves, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,2,2,4,0,6], k = 4
<strong>Output:</strong> 5
<strong>Explanation:</strong>
One of the ways we can end with 5 at the top of the pile after 4 moves is as follows:
- Step 1: Remove the topmost element = 5. The pile becomes [2,2,4,0,6].
- Step 2: Remove the topmost element = 2. The pile becomes [2,4,0,6].
- Step 3: Remove the topmost element = 2. The pile becomes [4,0,6].
- Step 4: Add 5 back onto the pile. The pile becomes [5,4,0,6].
Note that this is not the only way to end with 5 at the top of the pile. It can be shown that 5 is the largest answer possible after 4 moves.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong>
In the first move, our only option is to pop the topmost element of the pile.
Since it is not possible to obtain a non-empty pile after one move, we return -1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i], k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 && (k - nums.len() as i32) % 2 == 0 {
            -1
        } else if k == 0 {
            nums[0]
        } else if k < nums.len() as i32 {
            nums[k as usize].max(*nums.iter().take(k as usize - 1).max().unwrap_or(&0))
        } else {
            *nums.iter().take(k as usize - 1).max().unwrap()
        }
    }
}
```
