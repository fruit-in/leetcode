# 2488. Count Subarrays With Median K
You are given an array `nums` of size `n` consisting of **distinct** integers from `1` to `n` and a positive integer `k`.

Return *the number of non-empty subarrays in* `nums` *that have a **median** equal to* `k`.

#### Note:
* The median of an array is the **middle** element after sorting the array in **ascending** order. If the array is of even length, the median is the **left** middle element.
    * For example, the median of `[2,3,1,4]` is `2`, and the median of `[8,4,3,5,1]` is `4`.
* A subarray is a contiguous part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,2,1,4,5], k = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> The subarrays that have a median equal to 4 are: [4], [4,5] and [1,4,5].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,1], k = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> [3] is the only subarray that has a median equal to 3.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= nums[i], k <= n`
* The integers in `nums` are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        if !nums.contains(&k) {
            return 0;
        }

        let i = nums.iter().position(|&x| x == k).unwrap();
        let mut diff = 0;
        let mut count = HashMap::from([(0, 1)]);
        let mut ret = 1;

        for j in i + 1..nums.len() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            if diff == 0 || diff == 1 {
                ret += 1;
            }
            *count.entry(diff).or_insert(0) += 1;
        }

        diff = 0;

        for j in (0..i).rev() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            ret += count.get(&-diff).unwrap_or(&0);
            ret += count.get(&(1 - diff)).unwrap_or(&0);
        }

        ret
    }
}
```
