# 2317. Maximum XOR After Operations
You are given a **0-indexed** integer array `nums`. In one operation, select **any** non-negative integer `x` and an index `i`, then **update** `nums[i]` to be equal to `nums[i] AND (nums[i] XOR x)`.

Note that `AND` is the bitwise AND operation and `XOR` is the bitwise XOR operation.

Return *the **maximum** possible bitwise XOR of all elements of* `nums` *after applying the operation **any number** of times*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,2,4,6]
<strong>Output:</strong> 7
<strong>Explanation:</strong> Apply the operation with x = 4 and i = 3, num[3] = 6 AND (6 XOR 4) = 6 AND 2 = 2.
Now, nums = [3, 2, 4, 2] and the bitwise XOR of all the elements = 3 XOR 2 XOR 4 XOR 2 = 7.
It can be shown that 7 is the maximum possible bitwise XOR.
Note that other operations may be used to achieve a bitwise XOR of 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,9,2]
<strong>Output:</strong> 11
<strong>Explanation:</strong> Apply the operation zero times.
The bitwise XOR of all the elements = 1 XOR 2 XOR 3 XOR 9 XOR 2 = 11.
It can be shown that 11 is the maximum possible bitwise XOR.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc | x)
    }
}
```
