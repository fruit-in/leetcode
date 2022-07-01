# 2176. Count Equal and Divisible Pairs in an Array
Given a **0-indexed** integer array `nums` of length `n` and an integer `k`, return *the **number of pairs*** `(i, j)` *where* `0 <= i < j < n`, *such that* `nums[i] == nums[j]` *and* `(i * j)` *is divisible by* `k`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,1,2,2,2,1,3], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong>
There are 4 pairs that meet all the requirements:
- nums[0] == nums[6], and 0 * 6 == 0, which is divisible by 2.
- nums[2] == nums[3], and 2 * 3 == 6, which is divisible by 2.
- nums[2] == nums[4], and 2 * 4 == 8, which is divisible by 2.
- nums[3] == nums[4], and 3 * 4 == 12, which is divisible by 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since no value in nums is repeated, there are no pairs (i,j) that meet all the requirements.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i], k <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```
