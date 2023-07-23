# 2501. Longest Square Streak in an Array
You are given an integer array `nums`. A subsequence of `nums` is called a **square streak** if:

* The length of the subsequence is at least `2`, and
* **after** sorting the subsequence, each element (except the first element) is the **square** of the previous number.

Return *the length of the **longest square streak** in* `nums`, *or return* `-1` *if there is no **square streak***.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,3,6,16,8,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Choose the subsequence [4,16,2]. After sorting it, it becomes [2,4,16].
- 4 = 2 * 2.
- 16 = 4 * 4.
Therefore, [4,16,2] is a square streak.
It can be shown that every subsequence of length 4 is not a square streak.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,5,6,7]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no square streak in nums so return -1.
</pre>

#### Constraints:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>2 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut nums_set = nums.clone().into_iter().collect::<HashSet<_>>();
        let mut ret = -1;

        nums.sort_unstable();

        for &num in &nums {
            let mut length = 1;
            let mut x = num;

            while x < 317 && nums_set.remove(&(x * x)) {
                x *= x;
                length += 1;
            }

            if length > ret.max(1) {
                ret = length
            }
        }

        ret
    }
}
```
