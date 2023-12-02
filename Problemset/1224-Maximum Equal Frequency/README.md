# 1224. Maximum Equal Frequency
Given an array `nums` of positive integers, return the longest possible length of an array prefix of `nums`, such that it is possible to remove **exactly one** element from this prefix so that every number that has appeared in it will have the same number of occurrences.

If after removing one element there are no remaining elements, it's still considered that every appeared number has the same number of ocurrences (0).

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,2,1,1,5,3,3,5]
<strong>Output:</strong> 7
<strong>Explanation:</strong> For the subarray [2,2,1,1,5,3,3] of length 7, if we remove nums[4] = 5, we will get [2,2,1,1,3,3], so that each number will appear exactly twice.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1,2,2,2,3,3,3,4,4,4,5]
<strong>Output:</strong> 13
</pre>

#### Constraints:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count_nums = HashMap::new();
        let mut count_ocurrences = HashMap::new();

        for &num in &nums {
            *count_nums.entry(num).or_insert(0) += 1;
        }
        for &ocurrence in count_nums.values() {
            *count_ocurrences.entry(ocurrence).or_insert(0) += 1;
        }

        for i in (0..nums.len()).rev() {
            if count_ocurrences.len() == 1 {
                let (&k0, &v0) = count_ocurrences.iter().next().unwrap();

                if k0 == 1 || v0 == 1 {
                    return i as i32 + 1;
                }
            } else if count_ocurrences.len() == 2 {
                let (&k0, &v0) = count_ocurrences.iter().max().unwrap();
                let (&k1, &v1) = count_ocurrences.iter().min().unwrap();

                if (k1, v1) == (1, 1) || (k0 == k1 + 1 && v0 == 1) {
                    return i as i32 + 1;
                }
            }

            *count_ocurrences.get_mut(&count_nums[&nums[i]]).unwrap() -= 1;
            if count_ocurrences.get(&(count_nums[&nums[i]])) == Some(&0) {
                count_ocurrences.remove(&count_nums[&nums[i]]);
            }
            *count_nums.get_mut(&nums[i]).unwrap() -= 1;
            if count_nums[&nums[i]] > 0 {
                *count_ocurrences.entry(count_nums[&nums[i]]).or_insert(0) += 1;
            }
        }

        1
    }
}
```
