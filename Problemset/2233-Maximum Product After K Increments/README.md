# 2233. Maximum Product After K Increments
You are given an array of non-negative integers `nums` and an integer `k`. In one operation, you may choose **any** element from `nums` and **increment** it by `1`.

Return *the **maximum product** of* `nums` *after **at most*** `k` *operations*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>. Note that you should maximize the product before taking the modulo.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,4], k = 5
<strong>Output:</strong> 20
<strong>Explanation:</strong> Increment the first number 5 times.
Now nums = [5, 4], with a product of 5 * 4 = 20.
It can be shown that 20 is maximum product possible, so we return 20.
Note that there may be other ways to increment nums to have the maximum product.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [6,3,3,2], k = 2
<strong>Output:</strong> 216
<strong>Explanation:</strong> Increment the second number 1 time and increment the fourth number 1 time.
Now nums = [6, 4, 3, 3], with a product of 6 * 4 * 3 * 3 = 216.
It can be shown that 216 is maximum product possible, so we return 216.
Note that there may be other ways to increment nums to have the maximum product.
</pre>

#### Constraints:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(|num| -num).collect::<BinaryHeap<_>>();

        for _ in 0..k {
            *nums.peek_mut().unwrap() -= 1;
        }

        nums.iter()
            .fold(1, |acc, x| acc * (-x as i64) % 1_000_000_007) as i32
    }
}
```
