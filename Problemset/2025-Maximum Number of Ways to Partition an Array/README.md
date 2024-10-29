# 2025. Maximum Number of Ways to Partition an Array
You are given a **0-indexed** integer array `nums` of length `n`. The number of ways to **partition** `nums` is the number of `pivot` indices that satisfy both conditions:

* `1 <= pivot < n`
* `nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]`

You are also given an integer `k`. You can choose to change the value of **one** element of `nums` to `k`, or to leave the array **unchanged**.

Return *the **maximum** possible number of ways to **partition*** `nums` *to satisfy both conditions after changing **at most** one element*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,-1,2], k = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> One optimal approach is to change nums[0] to k. The array becomes [3,-1,2].
There is one way to partition the array:
- For pivot = 2, we have the partition [3,-1 | 2]: 3 + -1 == 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0,0], k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The optimal approach is to leave the array unchanged.
There are two ways to partition the array:
- For pivot = 1, we have the partition [0 | 0,0]: 0 == 0 + 0.
- For pivot = 2, we have the partition [0,0 | 0]: 0 + 0 == 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], k = -33
<strong>Output:</strong> 4
<strong>Explanation:</strong> One optimal approach is to change nums[2] to k. The array becomes [22,4,-33,-20,-15,15,-16,7,19,-10,0,-13,-14].
There are four ways to partition the array.
</pre>

#### Constraints:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= k, nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut sum = nums[0] as i64;
        let mut prefix_sum = nums[0] as i64;
        let mut suffix_sum = 0;
        let mut prefix_count = HashMap::from([(prefix_sum, 1)]);
        let mut suffix_count = HashMap::new();
        let mut ret = 0;

        for i in (1..n).rev() {
            sum += nums[i] as i64;
            suffix_sum += nums[i] as i64;
            *suffix_count.entry(suffix_sum).or_insert(0) += 1;
        }

        if sum % 2 == 0 {
            ret = *suffix_count.get(&(sum / 2)).unwrap_or(&0);
        }

        if (suffix_sum + k) % 2 == 0 {
            ret = ret.max(*suffix_count.get(&((suffix_sum + k) / 2)).unwrap_or(&0));
        }

        for i in 1..n {
            let new_sum = sum - nums[i] as i64 + k;

            *suffix_count.get_mut(&suffix_sum).unwrap() -= 1;
            suffix_sum -= nums[i] as i64;

            if new_sum % 2 == 0 {
                let x = *prefix_count.get(&(new_sum / 2)).unwrap_or(&0);
                let y = *suffix_count.get(&(new_sum / 2)).unwrap_or(&0);
                ret = ret.max(x + y);
            }

            prefix_sum += nums[i] as i64;
            *prefix_count.entry(prefix_sum).or_insert(0) += 1;
        }

        ret
    }
}
```
