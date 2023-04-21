# 2044. 统计按位或能得到最大值的子集数目
给你一个整数数组 `nums` ，请你找出 `nums` 子集 **按位或** 可能得到的 **最大值** ，并返回按位或能得到最大值的 **不同非空子集的数目** 。

如果数组 `a` 可以由数组 `b` 删除一些元素（或不删除）得到，则认为数组 `a` 是数组 `b` 的一个 **子集** 。如果选中的元素下标位置不一样，则认为两个子集 **不同** 。

对数组 `a` 执行 **按位或** ，结果等于 `a[0] OR a[1] OR ... OR a[a.length - 1]`（下标从 **0** 开始）。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 子集按位或能得到的最大值是 3 。有 2 个子集按位或可以得到 3 ：
- [3]
- [3,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2]
<strong>输出:</strong> 7
<strong>解释:</strong> [2,2,2] 的所有非空子集的按位或都可以得到 2 。总共有 2<sup>3</sup> - 1 = 7 个子集。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,2,1,5]
<strong>输出:</strong> 6
<strong>解释:</strong> 子集按位或可能的最大值是 7 。有 6 个子集按位或可以得到 7 ：
- [3,5]
- [3,1,5]
- [3,2,5]
- [3,2,1,5]
- [2,5]
- [2,1,5]
</pre>

#### 提示:
* `1 <= nums.length <= 16`
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut count0 = HashMap::from([(0, 1)]);
        let mut count1 = HashMap::new();

        for &num in &nums {
            count1.clear();

            for (&x, &y) in count0.iter() {
                count1.entry(x | num).and_modify(|c| *c += y).or_insert(y);
            }

            for (&x, &y) in count1.iter() {
                count0.entry(x).and_modify(|c| *c += y).or_insert(y);
            }
        }

        count0[&nums.iter().fold(0, |a, b| a | b)]
    }
}
```
