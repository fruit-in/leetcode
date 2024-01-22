# 1438. 绝对差不超过限制的最长连续子数组
给你一个整数数组 `nums` ，和一个表示限制的整数 `limit`，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 `limit` 。

如果不存在满足条件的子数组，则返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [8,2,4,7], limit = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 所有子数组如下：
[8] 最大绝对差 |8-8| = 0 <= 4.
[8,2] 最大绝对差 |8-2| = 6 > 4.
[8,2,4] 最大绝对差 |8-2| = 6 > 4.
[8,2,4,7] 最大绝对差 |8-2| = 6 > 4.
[2] 最大绝对差 |2-2| = 0 <= 4.
[2,4] 最大绝对差 |2-4| = 2 <= 4.
[2,4,7] 最大绝对差 |2-7| = 5 > 4.
[4] 最大绝对差 |4-4| = 0 <= 4.
[4,7] 最大绝对差 |4-7| = 3 <= 4.
[7] 最大绝对差 |7-7| = 0 <= 4.
因此，满足题意的最长子数组的长度为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,1,2,4,7,2], limit = 5
<strong>输出:</strong> 4
<strong>解释:</strong> 满足题意的最长子数组是 [2,4,7,2]，其最大绝对差 |2-7| = 5 <= 5 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [4,2,2,2,4,4,2,2], limit = 0
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= limit <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut i = 0;
        let mut ret = 1;

        for j in 0..nums.len() {
            max_heap.push((nums[j], j));
            min_heap.push((-nums[j], j));

            while let Some(&(x, k)) = max_heap.peek() {
                if x - nums[j] > limit {
                    max_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }
            while let Some(&(x, k)) = min_heap.peek() {
                if nums[j] + x > limit {
                    min_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }

            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
