# 1674. Minimum Moves to Make Array Complementary
You are given an integer array `nums` of **even** length `n` and an integer `limit`. In one move, you can replace any integer from `nums` with another integer between `1` and `limit`, inclusive.

The array `nums` is **complementary** if for all indices `i` (**0-indexed**), `nums[i] + nums[n - 1 - i]` equals the same number. For example, the array `[1,2,3,4]` is complementary because for all indices `i`, `nums[i] + nums[n - 1 - i] = 5`.

Return the ***minimum** number of moves required to make* `nums` ***complementary***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,4,3], limit = 4
<strong>Output:</strong> 1
<strong>Explanation:</strong> In 1 move, you can change nums to [1,2,2,3] (underlined elements are changed).
nums[0] + nums[3] = 1 + 3 = 4.
nums[1] + nums[2] = 2 + 2 = 4.
nums[2] + nums[1] = 2 + 2 = 4.
nums[3] + nums[0] = 3 + 1 = 4.
Therefore, nums[i] + nums[n-1-i] = 4 for every i, so nums is complementary.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,2,1], limit = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> In 2 moves, you can change nums to [2,2,2,2]. You cannot change any number to 3 since 3 > limit.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,1,2], limit = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> nums is already complementary.
</pre>

#### Constraints:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= limit <= 10<sup>5</sup></code>
* `n` is even.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let n = nums.len();
        let mut arr = vec![0; limit * 2 + 2];
        let mut prefix_sum = 0;
        let mut ret = i32::MAX;

        for i in 0..n / 2 {
            let x = nums[i] as usize;
            let y = nums[n - 1 - i] as usize;

            arr[2] += 2;
            arr[x.min(y) + 1] -= 1;
            arr[x.max(y) + limit + 1] += 1;
            arr[x + y] -= 1;
            arr[x + y + 1] += 1;
        }

        for i in 2..=limit * 2 {
            prefix_sum += arr[i];
            ret = ret.min(prefix_sum);
        }

        ret
    }
}
```
