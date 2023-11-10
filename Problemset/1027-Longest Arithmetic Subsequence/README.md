# 1027. Longest Arithmetic Subsequence
Given an array `nums` of integers, return *the length of the longest arithmetic subsequence in* `nums`.

**Note** that:

* A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
* A sequence `seq` is arithmetic if `seq[i + 1] - seq[i]` are all the same value (for `0 <= i < seq.length - 1`).

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,6,9,12]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The whole array is an arithmetic sequence with steps of length = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [9,4,7,2,10]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest arithmetic subsequence is [4,7,10].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [20,1,15,3,10,5,8]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest arithmetic subsequence is [20,15,10,5].
</pre>

#### Constraints:
* `2 <= nums.length <= 1000`
* `0 <= nums[i] <= 500`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut dp = vec![vec![0; max_num as usize * 2 + 1]; max_num as usize + 1];
        let mut ret = 2;

        for i in 0..nums.len() {
            for diff in -max_num..=max_num {
                if nums[i] - diff < 0 || nums[i] - diff > max_num {
                    dp[nums[i] as usize][(diff + max_num) as usize] = 1;
                } else {
                    dp[nums[i] as usize][(diff + max_num) as usize] =
                        dp[(nums[i] - diff) as usize][(diff + max_num) as usize] + 1;
                }
                ret = ret.max(dp[nums[i] as usize][(diff + max_num) as usize]);
            }
        }

        ret
    }
}
```
