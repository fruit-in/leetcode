# 1636. 按照频率将数组升序排序
给你一个整数数组 `nums` ，请你将数组按照每个值的频率 **升序** 排序。如果有多个值的频率相同，请你按照数值本身将它们 **降序** 排序。

请你返回排序后的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,2,2,2,3]
<strong>输出:</strong> [3,1,1,2,2,2]
<strong>解释:</strong> '3' 频率为 1，'1' 频率为 2，'2' 频率为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,1,3,2]
<strong>输出:</strong> [1,3,3,2,2]
<strong>解释:</strong> '2' 和 '3' 频率都为 2 ，所以它们之间按照数值本身降序排序。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-1,1,-6,4,5,-6,1,4,1]
<strong>输出:</strong> [5,-1,4,4,-6,-6,1,1,1]
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `-100 <= nums[i] <= 100`

## 题解 (Rust)

### 1. 排序
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();

        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.sort_by_key(|k| count.get(k).unwrap());

        nums
    }
}
```
