# 2170. 使数组变成交替数组的最少操作数
给你一个下标从 **0** 开始的数组 `nums` ，该数组由 `n` 个正整数组成。

如果满足下述条件，则数组 `nums` 是一个 **交替数组** ：
* `nums[i - 2] == nums[i]` ，其中 `2 <= i <= n - 1` 。
* `nums[i - 1] != nums[i]` ，其中 `1 <= i <= n - 1` 。

在一步 **操作** 中，你可以选择下标 `i` 并将 `nums[i]` **更改** 为 **任一** 正整数。

返回使数组变成交替数组的 **最少操作数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1,3,2,4,3]
<strong>输出:</strong> 3
<strong>解释:</strong>
使数组变成交替数组的方法之一是将该数组转换为 [3,1,3,1,3,1] 。
在这种情况下，操作数为 3 。
可以证明，操作数少于 3 的情况下，无法使数组变成交替数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,2,2,2]
<strong>输出:</strong> 2
<strong>解释:</strong>
使数组变成交替数组的方法之一是将该数组转换为 [1,2,1,2,1].
在这种情况下，操作数为 2 。
注意，数组不能转换成 [2,2,2,2,2] 。因为在这种情况下，nums[0] == nums[1]，不满足交替数组的条件。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut even_count = HashMap::new();
        let mut odd_count = HashMap::new();
        let mut even_max2 = [(0, 0), (0, 0)];
        let mut odd_max2 = [(0, 0), (0, 0)];

        for i in 0..n as usize {
            if i % 2 == 0 {
                *even_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == even_max2[0].0 {
                    even_max2[0].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[0].1 {
                    even_max2[1] = even_max2[0];
                    even_max2[0] = (nums[i], even_count[&nums[i]]);
                } else if nums[i] == even_max2[1].0 {
                    even_max2[1].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[1].1 {
                    even_max2[1] = (nums[i], even_count[&nums[i]]);
                }
            } else {
                *odd_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == odd_max2[0].0 {
                    odd_max2[0].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[0].1 {
                    odd_max2[1] = odd_max2[0];
                    odd_max2[0] = (nums[i], odd_count[&nums[i]]);
                } else if nums[i] == odd_max2[1].0 {
                    odd_max2[1].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[1].1 {
                    odd_max2[1] = (nums[i], odd_count[&nums[i]]);
                }
            }
        }

        if even_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                n / 2 - odd_max2[1].1
            } else {
                n / 2 - odd_max2[0].1
            }
        } else if odd_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                (n + 1) / 2 - even_max2[1].1
            } else {
                (n + 1) / 2 - even_max2[0].1
            }
        } else {
            if even_max2[0].0 == odd_max2[0].0 {
                n - (even_max2[0].1 + odd_max2[1].1).max(even_max2[1].1 + odd_max2[0].1)
            } else {
                n - even_max2[0].1 - odd_max2[0].1
            }
        }
    }
}
```
