# 2404. Most Frequent Even Element
Given an integer array `nums`, return *the most frequent even element*.

If there is a tie, return the **smallest** one. If there is no such element, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,2,2,4,4,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The even elements are 0, 2, and 4. Of these, 2 and 4 appear the most.
We return the smallest one, which is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,4,4,9,2,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> 4 is the even element appears the most.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [29,47,21,41,13,37,25,7]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no even element.
</pre>

#### Constraints:
* `1 <= nums.length <= 2000`
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_count = 0;
        let mut ret = -1;

        for num in nums.iter().filter(|&&x| x % 2 == 0) {
            count.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        for (&k, v) in count {
            if v > max_count || (v == max_count && k < ret) {
                max_count = v;
                ret = k;
            }
        }

        ret
    }
}
```
