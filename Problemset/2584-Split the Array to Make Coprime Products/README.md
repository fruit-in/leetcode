# 2584. Split the Array to Make Coprime Products
You are given a **0-indexed** integer array `nums` of length `n`.

A **split** at an index `i` where `0 <= i <= n - 2` is called **valid** if the product of the first `i + 1` elements and the product of the remaining elements are coprime.

* For example, if `nums = [2, 3, 3]`, then a split at the index `i = 0` is valid because `2` and `9` are coprime, while a split at the index `i = 1` is not valid because `6` and `3` are not coprime. A split at the index `i = 2` is not valid because `i == n - 1`.

Return *the smallest index* `i` *at which the array can be split validly or* `-1` *if there is no such split*.

Two values `val1` and `val2` are coprime if `gcd(val1, val2) == 1` where `gcd(val1, val2)` is the greatest common divisor of `val1` and `val2`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/14/second.PNG)
<pre>
<strong>Input:</strong> nums = [4,7,8,15,3,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The table above shows the values of the product of the first i + 1 elements, the remaining elements, and their gcd at each index i.
The only valid split is at index 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/12/14/capture.PNG)
<pre>
<strong>Input:</strong> nums = [4,7,15,8,3,5]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The table above shows the values of the product of the first i + 1 elements, the remaining elements, and their gcd at each index i.
There is no valid split.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut range = HashMap::new();
        let mut arr = vec![0; n + 1];
        let mut prefix_sum = 0;

        for i in 0..n {
            for x in 1..=nums[i].isqrt() {
                if nums[i] % x != 0 {
                    continue;
                }

                for y in [x, nums[i] / x] {
                    if !range.contains_key(&y) {
                        range.insert(y, [i, i]);
                    }
                    range.get_mut(&y).unwrap()[1] = i;
                }
            }
        }

        for (&k, &v) in range.iter() {
            if k == 1 || v[0] == v[1] {
                continue;
            }

            arr[v[0]] += 1;
            arr[v[1]] -= 1;
        }

        for i in 0..n - 1 {
            prefix_sum += arr[i];

            if prefix_sum == 0 {
                return i as i32;
            }
        }

        -1
    }
}
```
