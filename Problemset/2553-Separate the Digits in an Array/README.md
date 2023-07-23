# 2553. Separate the Digits in an Array
Given an array of positive integers `nums`, return *an array* `answer` *that consists of the digits of each integer in* `nums` *after separating them in **the same order** they appear in* `nums`.

To separate the digits of an integer is to get all the digits it has in the same order.

* For example, for the integer `10921`, the separation of its digits is `[1,0,9,2,1]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [13,25,83,77]
<strong>Output:</strong> [1,3,2,5,8,3,7,7]
<strong>Explanation:</strong>
- The separation of 13 is [1,3].
- The separation of 25 is [2,5].
- The separation of 83 is [8,3].
- The separation of 77 is [7,7].
answer = [1,3,2,5,8,3,7,7]. Note that answer contains the separations in the same order.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [7,1,3,9]
<strong>Output:</strong> [7,1,3,9]
<strong>Explanation:</strong> The separation of each integer in nums is itself.
answer = [7,1,3,9].
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];

        for num in nums {
            for digit in num.to_string().bytes() {
                ret.push((digit - b'0') as i32);
            }
        }

        ret
    }
}
```
