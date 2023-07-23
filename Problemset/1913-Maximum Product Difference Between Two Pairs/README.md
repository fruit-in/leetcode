# 1913. Maximum Product Difference Between Two Pairs
The **product difference** between two pairs `(a, b)` and `(c, d)` is defined as `(a * b) - (c * d)`.
* For example, the product difference between `(5, 6)` and `(2, 7)` is `(5 * 6) - (2 * 7) = 16`.

Given an integer array `nums`, choose four **distinct** indices `w`, `x`, `y`, and `z` such that the **product difference** between pairs `(nums[w], nums[x])` and `(nums[y], nums[z])` is **maximized**.

Return *the **maximum** such product difference*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,6,2,7,4]
<strong>Output:</strong> 34
<strong>Explanation:</strong> We can choose indices 1 and 3 for the first pair (6, 7) and indices 2 and 4 for the second pair (2, 4).
The product difference is (6 * 7) - (2 * 4) = 34.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,2,5,9,7,4,8]
<strong>Output:</strong> 64
<strong>Explanation:</strong> We can choose indices 3 and 6 for the first pair (9, 8) and indices 1 and 5 for the second pair (2, 4).
The product difference is (9 * 8) - (2 * 4) = 64.
</pre>

#### Constraints:
* <code>4 <= nums.length <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max = (i32::MIN, i32::MIN);
        let mut min = (i32::MAX, i32::MAX);

        for num in nums {
            if num >= max.0 {
                max = (num, max.0);
            } else if num > max.1 {
                max.1 = num;
            }
            if num <= min.0 {
                min = (num, min.0)
            } else if num < min.1 {
                min.1 = num;
            }
        }

        max.0 * max.1 - min.0 * min.1
    }
}
```
