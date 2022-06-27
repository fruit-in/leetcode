# 2006. 差的绝对值为 K 的数对数目
给你一个整数数组 `nums` 和一个整数 `k` ，请你返回数对 `(i, j)` 的数目，满足 `i < j` 且 `|nums[i] - nums[j]| == k` 。

`|x|` 的值定义为：
* 如果 `x >= 0` ，那么值为 `x` 。
* 如果 `x < 0` ，那么值为 `-x` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,2,1], k = 1
<strong>输出:</strong> 4
<strong>解释:</strong> 差的绝对值为 1 的数对为：
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
- [1,2,2,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3], k = 3
<strong>输出:</strong> 0
<strong>解释:</strong> 没有任何数对差的绝对值为 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,2,1,5,4], k = 2
<strong>输出:</strong> 3
<strong>解释:</strong> 差的绝对值为 2 的数对为：
- [3,2,1,5,4]
- [3,2,1,5,4]
- [3,2,1,5,4]
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 100`
* `1 <= k <= 99`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            ret += count.get(&(num + k)).unwrap_or(&0);
            ret += count.get(&(num - k)).unwrap_or(&0);
            *count.entry(num).or_insert(0) += 1;
        }

        ret
    }
}
```
