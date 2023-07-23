# 2455. Average Value of Even Numbers That Are Divisible by Three
Given an integer array `nums` of **positive** integers, return *the average value of all even integers that are divisible by* `3`.

Note that the **average** of `n` elements is the **sum** of the `n` elements divided by `n` and **rounded down** to the nearest integer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,6,10,12,15]
<strong>Output:</strong> 9
<strong>Explanation:</strong> 6 and 12 are even numbers that are divisible by 3. (6 + 12) / 2 = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,4,7,10]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no single number that satisfies the requirement, so return 0.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().filter(|&&x| x % 6 == 0).sum::<i32>();
        let len = nums.iter().filter(|&&x| x % 6 == 0).count() as i32;

        sum.checked_div(len).unwrap_or(0)
    }
}
```
