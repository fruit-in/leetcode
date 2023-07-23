# 2598. Smallest Missing Non-negative Integer After Operations
You are given a **0-indexed** integer array `nums` and an integer `value`.

In one operation, you can add or subtract `value` from any element of `nums`.

* For example, if `nums = [1,2,3]` and `value = 2`, you can choose to subtract `value` from `nums[0]` to make `nums = [-1,2,3]`.

The MEX (minimum excluded) of an array is the smallest missing **non-negative** integer in it.

* For example, the MEX of `[-1,2,3]` is `0` while the MEX of `[1,0,3]` is `2`.

Return *the maximum MEX of* `nums` *after applying the mentioned operation **any number of times***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,-10,7,13,6,8], value = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> One can achieve this result by applying the following operations:
- Add value to nums[1] twice to make nums = [1,0,7,13,6,8]
- Subtract value from nums[2] once to make nums = [1,0,2,13,6,8]
- Subtract value from nums[3] twice to make nums = [1,0,2,3,6,8]
The MEX of nums is 4. It can be shown that 4 is the maximum MEX we can achieve.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,-10,7,13,6,8], value = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> One can achieve this result by applying the following operation:
- subtract value from nums[2] once to make nums = [1,-10,0,13,6,8]
The MEX of nums is 2. It can be shown that 2 is the maximum MEX we can achieve.
</pre>

#### Constraints:
* <code>1 <= nums.length, value <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut count = vec![0; value as usize];

        for &num in &nums {
            count[num.rem_euclid(value) as usize] += 1;
        }

        for num in 0..=nums.len() as i32 {
            if count[num.rem_euclid(value) as usize] == 0 {
                return num;
            }

            count[num.rem_euclid(value) as usize] -= 1;
        }

        unreachable!();
    }
}
```
