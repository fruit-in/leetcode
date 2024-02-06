# 1802. Maximum Value at a Given Index in a Bounded Array
You are given three positive integers: `n`, `index`, and `maxSum`. You want to construct an array `nums` (**0-indexed**) that satisfies the following conditions:

* `nums.length == n`
* `nums[i]` is a **positive** integer where `0 <= i < n`.
* `abs(nums[i] - nums[i+1]) <= 1` where `0 <= i < n-1`.
* The sum of all the elements of `nums` does not exceed `maxSum`.
* `nums[index]` is **maximized**.

Return `nums[index]` *of the constructed array*.

Note that `abs(x)` equals `x` if `x >= 0`, and `-x` otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4, index = 2,  maxSum = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> nums = [1,2,2,1] is one array that satisfies all the conditions.
There are no arrays that satisfy all the conditions and have nums[2] == 3, so 2 is the maximum nums[2].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6, index = 1,  maxSum = 10
<strong>Output:</strong> 3
</pre>

#### Constraints:
* <code>1 <= n <= maxSum <= 10<sup>9</sup></code>
* `0 <= index < n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        if index < n - index - 1 {
            return Self::max_value(n, n - index - 1, max_sum);
        }

        let n = n as i64;
        let index = index as i64;
        let max_sum = max_sum as i64 - n;
        let mut min = 0;
        let mut max = max_sum;

        while min < max {
            let x = (min + max + 1) / 2;
            let sum = if x > index {
                ((2 * x - index) * (index + 1) + (2 * x - n + index) * (n - 1 - index)) / 2
            } else if x > n - 1 - index {
                ((1 + x) * x + (2 * x - n + index) * (n - 1 - index)) / 2
            } else {
                x * x
            };

            if sum <= max_sum {
                min = x;
            } else {
                max = x - 1;
            }
        }

        min as i32 + 1
    }
}
```
