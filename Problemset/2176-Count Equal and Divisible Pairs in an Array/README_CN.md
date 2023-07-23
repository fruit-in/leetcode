# 2176. 统计数组中相等且可以被整除的数对
给你一个下标从 **0** 开始长度为 `n` 的整数数组 `nums` 和一个整数 `k` ，请你返回满足 `0 <= i < j < n` ，`nums[i] == nums[j]` 且 `(i * j)` 能被 `k` 整除的数对 `(i, j)` 的 **数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1,2,2,2,1,3], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong>
总共有 4 对数符合所有要求：
- nums[0] == nums[6] 且 0 * 6 == 0 ，能被 2 整除。
- nums[2] == nums[3] 且 2 * 3 == 6 ，能被 2 整除。
- nums[2] == nums[4] 且 2 * 4 == 8 ，能被 2 整除。
- nums[3] == nums[4] 且 3 * 4 == 12 ，能被 2 整除。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 由于数组中没有重复数值，所以没有数对 (i,j) 符合所有要求。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i], k <= 100`

## 题解 (Rust)

### 1. 题解
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
