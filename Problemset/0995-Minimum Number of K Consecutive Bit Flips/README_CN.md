# 995. K 连续位的最小翻转次数
给定一个二进制数组 `nums` 和一个整数 `k` 。

**k位翻转** 就是从 `nums` 中选择一个长度为 `k` 的 **子数组** ，同时把子数组中的每一个 `0` 都改成 `1` ，把子数组中的每一个 `1` 都改成 `0` 。

返回数组中不存在 `0` 所需的最小 **k位翻转** 次数。如果不可能，则返回 `-1` 。

**子数组** 是数组的 **连续** 部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,0], k = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 先翻转 A[0]，然后翻转 A[2]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,0], k = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 无论我们怎样翻转大小为 2 的子数组，我们都不能使数组变为 [1,1,1]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0,0,0,1,0,1,1,0], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong>
翻转 A[0],A[1],A[2]: A变成 [1,1,1,1,0,1,1,0]
翻转 A[4],A[5],A[6]: A变成 [1,1,1,1,1,0,0,0]
翻转 A[5],A[6],A[7]: A变成 [1,1,1,1,1,1,1,1]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
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
