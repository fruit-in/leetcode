# 2302. 统计得分小于 K 的子数组数目
一个数组的 **分数** 定义为数组之和 **乘以** 数组的长度。

* 比方说，`[1, 2, 3, 4, 5]` 的分数为 `(1 + 2 + 3 + 4 + 5) * 5 = 75` 。

给你一个正整数数组 `nums` 和一个整数 `k` ，请你返回 `nums` 中分数 **严格小于** `k` 的 **非空整数子数组数目**。

**子数组** 是数组中的一个连续元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,4,3,5], k = 10
<strong>输出:</strong> 6
<strong>解释:</strong>
有 6 个子数组的分数小于 10 ：
- [2] 分数为 2 * 1 = 2 。
- [1] 分数为 1 * 1 = 1 。
- [4] 分数为 4 * 1 = 4 。
- [3] 分数为 3 * 1 = 3 。
- [5] 分数为 5 * 1 = 5 。
- [2,1] 分数为 (2 + 1) * 2 = 6 。
注意，子数组 [1,4] 和 [4,3,5] 不符合要求，因为它们的分数分别为 10 和 36，但我们要求子数组的分数严格小于 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1], k = 5
<strong>输出:</strong> 5
<strong>解释:</strong>
除了 [1,1,1] 以外每个子数组分数都小于 5 。
[1,1,1] 分数为 (1 + 1 + 1) * 3 = 9 ，大于 5 。
所以总共有 5 个子数组得分小于 5 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>15</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        for i in 0..nums.len() {
            let mut lo = 0;
            let mut hi = i + 1;

            while lo < hi {
                let mid = (lo + hi) / 2;
                let score = (prefix_sum[i + 1] - prefix_sum[mid]) * (i + 1 - mid) as i64;

                if score < k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            ret += (i + 1 - hi) as i64;
        }

        ret
    }
}
```
