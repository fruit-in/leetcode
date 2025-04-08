# 2537. 统计好子数组的数目
给你一个整数数组 `nums` 和一个整数 `k` ，请你返回 `nums` 中 **好** 子数组的数目。

一个子数组 `arr` 如果有 **至少** `k` 对下标 `(i, j)` 满足 `i < j` 且 `arr[i] == arr[j]` ，那么称它是一个 **好** 子数组。

**子数组** 是原数组中一段连续 **非空** 的元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], k = 10
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一的好子数组是这个数组本身。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,1,4,3,2,2,4], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 4 个不同的好子数组：
- [3,1,4,3,2,2] 有 2 对。
- [3,1,4,3,2,2,4] 有 3 对。
- [1,4,3,2,2,4] 有 2 对。
- [4,3,2,2,4] 有 2 对。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = HashMap::new();
        let mut pairs = 0;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *count.entry(nums[j]).or_insert(0) += 1;
            pairs += count[&nums[j]] - 1;

            while pairs >= k {
                *count.get_mut(&nums[i]).unwrap() -= 1;
                pairs -= count[&nums[i]];
                i += 1;
            }

            ret += i as i64;
        }

        ret
    }
}
```
