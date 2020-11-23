# 1590. Make Sum Divisible by P
Given an array of positive integers `nums`, remove the **smallest** subarray (possibly **empty**) such that the **sum** of the remaining elements is divisible by `p`. It is **not** allowed to remove the whole array.

Return *the length of the smallest subarray that you need to remove, or* `-1` *if it's impossible*.

A **subarray** is defined as a contiguous block of elements in the array.

#### Example 1:
<pre>
<b>Input:</b> nums = [3,1,4,2], p = 6
<b>Output:</b> 1
<b>Explanation:</b> The sum of the elements in nums is 10, which is not divisible by 6. We can remove the subarray [4], and the sum of the remaining elements is 6, which is divisible by 6.
</pre>

#### Example 2:
<pre>
<b>Input:</b> nums = [6,3,5,2], p = 9
<b>Output:</b> 2
<b>Explanation:</b> We cannot remove a single element to get a sum divisible by 9. The best way is to remove the subarray [5,2], leaving us with [6,3] with sum 9.
</pre>

#### Example 3:
<pre>
<b>Input:</b> nums = [1,2,3], p = 3
<b>Output:</b> 0
<b>Explanation:</b> Here the sum is 6. which is already divisible by 3. Thus we do not need to remove anything.
</pre>

#### Example 4:
<pre>
<b>Input:</b> nums = [1,2,3], p = 7
<b>Output:</b> -1
<b>Explanation:</b> There is no way to remove a subarray in order to get a sum divisible by 7.
</pre>

#### Example 5:
<pre>
<b>Input:</b> nums = [1000000000,1000000000,1000000000], p = 3
<b>Output:</b> 0
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= p <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Prefix Sum
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let mut prefix = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut hash = HashMap::new();
        let mut ret = std::i32::MAX;

        for i in 1..prefix.len() {
            prefix[i] += prefix[i - 1];
        }

        let sum = *prefix.last().unwrap();

        for i in 0..prefix.len() {
            hash.insert(prefix[i] % p, i);
            if prefix[i] % p == 0 {
                ret = ret.min((prefix.len() - 1 - i) as i32);
            }
            if (sum - prefix[i]) % p == 0 {
                match hash.get(&0) {
                    Some(&j) => ret = ret.min((i - j) as i32),
                    None => ret = ret.min(i as i32 + 1),
                }
            }
            if let Some(&j) = hash.get(&(p - (sum - prefix[i]) % p)) {
                ret = ret.min((i - j) as i32);
            }
        }

        if ret == prefix.len() as i32 {
            return -1;
        }

        ret
    }
}
```
