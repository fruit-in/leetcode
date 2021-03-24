# 560. 和为K的子数组
给定一个整数数组和一个整数 **k**，你需要找到该数组中和为 **k** 的连续的子数组的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1], k = 2
<strong>输出:</strong> 2 , [1,1] 与 [1,1] 为两种不同的情况。
</pre>
</pre>

#### 说明:
1. 数组的长度为 [1, 20,000]。
2. 数组中元素的范围是 [-1000, 1000] ，且整数 **k** 的范围是 [-1e7, 1e7]。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for num in nums {
            sum += num;
            ret += count.get(&(sum - k)).unwrap_or(&0);
            *count.entry(sum).or_insert(0) += 1;
        }

        ret
    }
}
```
