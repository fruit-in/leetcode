# 2256. Minimum Average Difference
You are given a **0-indexed** integer array `nums` of length `n`.

The **average difference** of the index `i` is the **absolute difference** between the average of the **first** `i + 1` elements of `nums` and the average of the **last** `n - i - 1` elements. Both averages should be **rounded down** to the nearest integer.

Return *the index with the **minimum average difference***. If there are multiple such indices, return the **smallest** one.

**Note:**

* The **absolute difference** of two numbers is the absolute value of their difference.
* The **average** of `n` elements is the sum of the `n` elements divided (**integer division**) by `n`.
* The average of `0` elements is considered to be `0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,5,3,9,5,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- The average difference of index 0 is: |2 / 1 - (5 + 3 + 9 + 5 + 3) / 5| = |2 / 1 - 25 / 5| = |2 - 5| = 3.
- The average difference of index 1 is: |(2 + 5) / 2 - (3 + 9 + 5 + 3) / 4| = |7 / 2 - 20 / 4| = |3 - 5| = 2.
- The average difference of index 2 is: |(2 + 5 + 3) / 3 - (9 + 5 + 3) / 3| = |10 / 3 - 17 / 3| = |3 - 5| = 2.
- The average difference of index 3 is: |(2 + 5 + 3 + 9) / 4 - (5 + 3) / 2| = |19 / 4 - 8 / 2| = |4 - 4| = 0.
- The average difference of index 4 is: |(2 + 5 + 3 + 9 + 5) / 5 - 3 / 1| = |24 / 5 - 3 / 1| = |4 - 3| = 1.
- The average difference of index 5 is: |(2 + 5 + 3 + 9 + 5 + 3) / 6 - 0| = |27 / 6 - 0| = |4 - 0| = 4.
The average difference of index 3 is the minimum average difference so return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
The only index is 0 so return 0.
The average difference of index 0 is: |0 / 1 - 0| = |0 - 0| = 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut prefix_sum = 0;
        let mut suffix_sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut min_avg_diff = i64::MAX;
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;
            suffix_sum -= nums[i] as i64;
            let prefix_avg = prefix_sum / (i as i64 + 1);
            let suffix_avg = suffix_sum.checked_div(n - i as i64 - 1).unwrap_or(0);
            let avg_diff = (prefix_avg - suffix_avg).abs();

            if avg_diff < min_avg_diff {
                min_avg_diff = avg_diff;
                ret = i;
            }
        }

        ret as i32
    }
}
```
