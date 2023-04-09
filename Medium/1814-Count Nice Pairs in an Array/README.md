# 1814. Count Nice Pairs in an Array
You are given an array `nums` that consists of non-negative integers. Let us define `rev(x)` as the reverse of the non-negative integer `x`. For example, `rev(123) = 321`, and `rev(120) = 21`. A pair of indices `(i, j)` is **nice** if it satisfies all of the following conditions:

* `0 <= i < j < nums.length`
* `nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])`

Return *the number of nice pairs of indices*. Since that number can be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [42,11,1,97]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two pairs are:
 - (0,3) : 42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121.
 - (1,2) : 11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [13,10,35,24,76]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* <code>1 <= nums.length <=10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for &num in &nums {
            count
                .entry(num - Self::rev(num))
                .and_modify(|x| *x += 1)
                .or_insert(1_i64);
        }

        (count.into_values().map(|x| (x - 1) * x / 2).sum::<i64>() % 1_000_000_007) as i32
    }

    pub fn rev(x: i32) -> i32 {
        let mut x = x;
        let mut ret = 0;

        while x > 0 {
            ret = ret * 10 + x % 10;
            x /= 10;
        }

        ret
    }
}
```
