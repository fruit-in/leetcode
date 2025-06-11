# 992. K 个不同整数的子数组
给定一个正整数数组 `nums`和一个整数 `k`，返回 `nums` 中 **「好子数组」** 的数目。

如果 `nums` 的某个子数组中不同整数的个数恰好为 `k`，则称 `nums` 的这个连续、不一定不同的子数组为 **「好子数组 」**。

* 例如，`[1,2,3,1,2]` 中有 `3` 个不同的整数：`1`，`2`，以及 `3`。

**子数组** 是数组的 **连续** 部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,3], k = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 恰好由 2 个不同整数组成的子数组：[1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2].
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,1,3,4], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 恰好由 3 个不同整数组成的子数组：[1,2,1,3], [2,1,3], [1,3,4].
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* `1 <= nums[i], k <= nums.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut count = HashMap::new();
        let mut max_index = HashMap::new();
        let mut l = 0;
        let mut m = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            *count.entry(nums[r]).or_insert(0) += 1;
            *max_index.entry(nums[r]).or_insert(r) = r;

            while count.len() > k {
                *count.get_mut(&nums[l]).unwrap() -= 1;
                if *count.get(&nums[l]).unwrap() == 0 {
                    count.remove(&nums[l]);
                }
                l += 1;
            }

            if count.len() == k {
                while m < l || max_index[&nums[m]] != m {
                    m += 1;
                }
                ret += m - l + 1;
            }
        }

        ret as i32
    }
}
```
