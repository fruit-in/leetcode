# 2342. Max Sum of a Pair With Equal Sum of Digits
You are given a **0-indexed** array `nums` consisting of **positive** integers. You can choose two indices `i` and `j`, such that `i != j`, and the sum of digits of the number `nums[i]` is equal to that of `nums[j]`.

Return *the **maximum** value of* `nums[i] + nums[j]` *that you can obtain over all possible indices* `i` *and* `j` *that satisfy the conditions*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [18,43,36,13,7]
<strong>Output:</strong> 54
<strong>Explanation:</strong> The pairs (i, j) that satisfy the conditions are:
- (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
- (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
So the maximum sum that we can obtain is 54.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,12,19,14]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are no two numbers that satisfy the conditions, so we return -1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_pairs = HashMap::new();

        for &num in &nums {
            let digits_sum = num
                .to_string()
                .bytes()
                .map(|d| (d - b'0') as i32)
                .sum::<i32>();
            let pair = max_pairs.entry(digits_sum).or_insert((0, 0));

            if num >= pair.1 {
                *pair = (pair.1, num);
            } else if num > pair.0 {
                *pair = (num, pair.1);
            }
        }

        max_pairs
            .values()
            .filter(|&&(x, y)| x > 0)
            .map(|(x, y)| x + y)
            .max()
            .unwrap_or(-1)
    }
}
```
