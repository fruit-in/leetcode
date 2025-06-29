# 2261. K Divisible Elements Subarrays
Given an integer array `nums` and two integers `k` and `p`, return *the number of **distinct subarrays**, which have **at most*** `k` *elements that are divisible by* `p`.

Two arrays `nums1` and `nums2` are said to be **distinct** if:
* They are of **different** lengths, or
* There exists **at least** one index `i` where `nums1[i] != nums2[i]`.

A **subarray** is defined as a **non-empty** contiguous sequence of elements in an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,3,2,2], k = 2, p = 2
<strong>Output:</strong> 11
<strong>Explanation:</strong>
The elements at indices 0, 3, and 4 are divisible by p = 2.
The 11 distinct subarrays which have at most k = 2 elements divisible by 2 are:
[2], [2,3], [2,3,3], [2,3,3,2], [3], [3,3], [3,3,2], [3,3,2,2], [3,2], [3,2,2], and [2,2].
Note that the subarrays [2] and [3] occur more than once in nums, but they should each be counted only once.
The subarray [2,3,3,2,2] should not be counted because it has 3 elements that are divisible by 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 4, p = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong>
All element of nums are divisible by p = 1.
Also, every subarray of nums will have at most 4 elements that are divisible by 1.
Since all subarrays are distinct, the total number of subarrays satisfying all the constraints is 10.
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* `1 <= nums[i], p <= 200`
* `1 <= k <= nums.length`

#### Follow up:
Can you solve this problem in O(n<sup>2</sup>) time complexity?

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        const BASE: i64 = 257;
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        let mut rolling_hash1 = HashSet::new();
        let mut rolling_hash2 = HashSet::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            let mut count = 0;
            let mut hash1 = 0;
            let mut hash2 = 0;

            for j in i..nums.len() {
                if nums[j] % p == 0 {
                    count += 1;
                }
                if count > k {
                    break;
                }

                hash1 = (hash1 * BASE + nums[j] as i64) % MOD1;
                hash2 = (hash2 * BASE + nums[j] as i64) % MOD2;

                if !rolling_hash1.contains(&hash1) || !rolling_hash2.contains(&hash2) {
                    ret += 1;
                }

                rolling_hash1.insert(hash1);
                rolling_hash2.insert(hash2);
            }
        }

        ret
    }
}
```
