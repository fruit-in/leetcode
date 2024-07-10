# 1856. Maximum Subarray Min-Product
The **min-product** of an array is equal to the **minimum value** in the array **multiplied by** the array's **sum**.

* For example, the array `[3,2,5]` (minimum value is `2`) has a min-product of `2 * (3+2+5) = 2 * 10 = 20`.

Given an array of integers `nums`, return *the **maximum min-product** of any **non-empty subarray** of* `nums`. Since the answer may be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

Note that the min-product should be maximized **before** performing the modulo operation. Testcases are generated such that the maximum min-product **without** modulo will fit in a **64-bit signed integer**.

A **subarray** is a **contiguous** part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,2]
<strong>Output:</strong> 14
<strong>Explanation:</strong> The maximum min-product is achieved with the subarray [2,3,2] (minimum value is 2).
2 * (2+3+2) = 2 * 7 = 14.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,3,1,2]
<strong>Output:</strong> 18
<strong>Explanation:</strong> The maximum min-product is achieved with the subarray [3,3] (minimum value is 3).
3 * (3+3) = 3 * 6 = 18.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,1,5,6,4,2]
<strong>Output:</strong> 60
<strong>Explanation:</strong> The maximum min-product is achieved with the subarray [5,6,4] (minimum value is 4).
4 * (5+6+4) = 4 * 15 = 60.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut stack = vec![];
        let mut indexl = vec![None; n];
        let mut indexr = vec![None; n];
        let mut ret = nums[0] as i64 * nums[0] as i64;

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;

            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexl[i] = Some(j);
                    break;
                }
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexr[i] = Some(j);
                    break;
                }
            }
            stack.push(i);

            let mut sum = prefix_sum[indexr[i].unwrap_or(n)];
            if let Some(j) = indexl[i] {
                sum -= prefix_sum[j + 1];
            }
            ret = ret.max(nums[i] as i64 * sum);
        }

        (ret % 1_000_000_007) as i32
    }
}
```
