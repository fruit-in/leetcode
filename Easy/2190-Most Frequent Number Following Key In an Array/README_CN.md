# 2190. 数组中紧跟 key 之后出现最频繁的数字
给你一个下标从 **0** 开始的整数数组 `nums` ，同时给你一个整数 `key` ，它在 `nums` 出现过。

**统计** 在 `nums` 数组中紧跟着 `key` 后面出现的不同整数 `target` 的出现次数。换言之，`target` 的出现次数为满足以下条件的 `i` 的数目：
* `0 <= i <= n - 2`
* `nums[i] == key` 且
* `nums[i + 1] == target` 。

请你返回出现 **最多** 次数的 `target` 。测试数据保证出现次数最多的 `target` 是唯一的。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,100,200,1,100], key = 1
<strong>输出:</strong> 100
<strong>解释:</strong> 对于 target = 100 ，在下标 1 和 4 处出现过 2 次，且都紧跟着 key 。
没有其他整数在 key 后面紧跟着出现，所以我们返回 100 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2,2,3], key = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 对于 target = 2 ，在下标 1 ，2 和 3 处出现过 3 次，且都紧跟着 key 。
对于 target = 3 ，在下标 4 出出现过 1 次，且紧跟着 key 。
target = 2 是紧跟着 key 之后出现次数最多的数字，所以我们返回 2 。
</pre>

#### 提示:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`
* 测试数据保证答案是唯一的。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = HashMap::new();
        let mut max = 0;
        let mut ret = 0;

        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                *count.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }

        count.into_iter().map(|(k, v)| (v, k)).max().unwrap().1
    }
}
```
