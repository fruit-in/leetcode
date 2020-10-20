# 347. 前 K 个高频元素
给定一个非空的整数数组，返回其中出现频率前 ***k*** 高的元素。

#### 示例 1:
<pre>
<b>输入:</b> nums = [1,1,1,2,2,3], k = 2
<b>输出:</b> [1,2]
</pre>

#### 示例 2:
<pre>
<b>输入:</b> nums = [1], k = 1
<b>输出:</b> [1]
</pre>

#### 提示:
* 你可以假设给定的 *k* 总是合理的，且 1 ≤ k ≤ 数组中不相同的元素的个数。
* 你的算法的时间复杂度**必须**优于 O(*n* log *n*) , *n* 是数组的大小。
* 题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的。
* 你可以按任意顺序返回答案。

## 题解 (Rust)

### 1. 排序
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter = HashMap::new();

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut counter = counter.iter().collect::<Vec<_>>();

        counter.sort_unstable_by_key(|c| c.1);

        counter
            .iter()
            .rev()
            .take(k as usize)
            .map(|(k, v)| **k)
            .collect()
    }
}
```
