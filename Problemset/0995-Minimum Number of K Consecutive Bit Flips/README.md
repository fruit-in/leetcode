# 995. Minimum Number of K Consecutive Bit Flips
You are given a binary array `nums` and an integer `k`.

A **k-bit** flip is choosing a **subarray** of length `k` from `nums` and simultaneously changing every `0` in the subarray to `1`, and every `1` in the subarray to `0`.

Return *the minimum number of **k-bit flips** required so that there is no* `0` *in the array*. If it is not possible, return `-1`.

A **subarray** is a **contiguous** part of an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,0], k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> Flip nums[0], then flip nums[2].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,0], k = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> No matter how we flip subarrays of size 2, we cannot make the array become [1,1,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0,0,0,1,0,1,1,0], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Flip nums[0],nums[1],nums[2]: nums becomes [1,1,1,1,0,1,1,0]
Flip nums[4],nums[5],nums[6]: nums becomes [1,1,1,1,1,0,0,0]
Flip nums[5],nums[6],nums[7]: nums becomes [1,1,1,1,1,1,1,1]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `1 <= k <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut flip = vec![false; nums.len()];
        let mut window_xor = false;
        let mut ret = 0;

        for i in 0..nums.len() {
            if (nums[i] == 0 && !window_xor) || (nums[i] == 1 && window_xor) {
                if i + k - 1 < nums.len() {
                    flip[i] = true;
                    ret += 1;
                } else {
                    return -1;
                }
            }
            window_xor ^= flip[i];
            if i >= k - 1 {
                window_xor ^= flip[i - k + 1];
            }
        }

        ret
    }
}
```
