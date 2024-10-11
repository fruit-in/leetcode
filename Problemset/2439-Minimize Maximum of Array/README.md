# 2439. Minimize Maximum of Array
You are given a **0-indexed** array `nums` comprising of `n` non-negative integers.

In one operation, you must:

* Choose an integer `i` such that `1 <= i < n` and `nums[i] > 0`.
* Decrease `nums[i]` by 1.
* Increase `nums[i - 1]` by 1.

Return *the **minimum** possible value of the **maximum** integer of* `nums` *after performing **any** number of operations*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,7,1,6]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
One set of optimal operations is as follows:
1. Choose i = 1, and nums becomes [4,6,1,6].
2. Choose i = 3, and nums becomes [4,6,2,5].
3. Choose i = 1, and nums becomes [5,5,2,5].
The maximum integer of nums is 5. It can be shown that the maximum number cannot be less than 5.
Therefore, we return 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,1]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
It is optimal to leave nums as is, and since 10 is the maximum value, we return 10.
</pre>

#### Constraints:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lo = *nums.iter().min().unwrap() as i64;
        let mut hi = *nums.iter().max().unwrap() as i64;

        while lo < hi {
            let m = (lo + hi) / 2;
            let mut x = nums[n - 1] as i64;

            for i in (1..n).rev() {
                if x > m {
                    x += nums[i - 1] as i64 - m;
                } else {
                    x = nums[i - 1] as i64;
                }
            }

            if x > m {
                lo = m + 1;
            } else {
                hi = m;
            }
        }

        hi as i32
    }
}
```
