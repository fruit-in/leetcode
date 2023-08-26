# 2535. Difference Between Element Sum and Digit Sum of an Array
You are given a positive integer array `nums`.

* The **element sum** is the sum of all the elements in `nums`.
* The **digit sum** is the sum of all the digits (not necessarily distinct) that appear in `nums`.

Return *the **absolute** difference between the **element sum** and **digit sum** of* `nums`.

**Note** that the absolute difference between two integers `x` and `y` is defined as `|x - y|`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,15,6,3]
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The element sum of nums is 1 + 15 + 6 + 3 = 25.
The digit sum of nums is 1 + 1 + 5 + 6 + 3 = 16.
The absolute difference between the element sum and digit sum is |25 - 16| = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
The element sum of nums is 1 + 2 + 3 + 4 = 10.
The digit sum of nums is 1 + 2 + 3 + 4 = 10.
The absolute difference between the element sum and digit sum is |10 - 10| = 0.
</pre>

#### Constraints:
* `1 <= nums.length <= 2000`
* `1 <= nums[i] <= 2000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut element_sum = 0;
        let mut digit_sum = 0;

        for i in 0..nums.len() {
            element_sum += nums[i];
            while nums[i] > 0 {
                digit_sum += nums[i] % 10;
                nums[i] /= 10;
            }
        }

        (element_sum - digit_sum).abs()
    }
}
```
