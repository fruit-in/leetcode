# 2261. 含最多 K 个可整除元素的子数组
给你一个整数数组 `nums` 和两个整数 `k` 和 `p` ，找出并返回满足要求的不同的子数组数，要求子数组中最多 `k` 个可被 `p` 整除的元素。

如果满足下述条件之一，则认为数组 `nums1` 和 `nums2` 是 **不同** 数组：
* 两数组长度 **不同** ，或者
* 存在 **至少** 一个下标 `i` 满足 `nums1[i] != nums2[i]` 。

**子数组** 定义为：数组中的连续元素组成的一个 **非空** 序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,3,2,2], k = 2, p = 2
<strong>输出:</strong> 11
<strong>解释:</strong>
位于下标 0、3 和 4 的元素都可以被 p = 2 整除。
共计 11 个不同子数组都满足最多含 k = 2 个可以被 2 整除的元素：
[2]、[2,3]、[2,3,3]、[2,3,3,2]、[3]、[3,3]、[3,3,2]、[3,3,2,2]、[3,2]、[3,2,2] 和 [2,2] 。
注意，尽管子数组 [2] 和 [3] 在 nums 中出现不止一次，但统计时只计数一次。
子数组 [2,3,3,2,2] 不满足条件，因为其中有 3 个元素可以被 2 整除。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 4, p = 1
<strong>输出:</strong> 10
<strong>解释:</strong>
nums 中的所有元素都可以被 p = 1 整除。
此外，nums 中的每个子数组都满足最多 4 个元素可以被 1 整除。
因为所有子数组互不相同，因此满足所有限制条件的子数组总数为 10 。
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i], p <= 200`
* `1 <= k <= nums.length`

#### 进阶:
你可以设计并实现时间复杂度为 <code>O(n<sup>2</sup>)</code> 的算法解决此问题吗？

## 题解 (Rust)

### 1. 题解
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
