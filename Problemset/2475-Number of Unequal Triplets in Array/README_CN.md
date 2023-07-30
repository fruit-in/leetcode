# 2475. 数组中不等三元组的数目
给你一个下标从 **0** 开始的正整数数组 `nums` 。请你找出并统计满足下述条件的三元组 `(i, j, k)` 的数目：

* `0 <= i < j < k < nums.length`
* `nums[i]`、`nums[j]` 和 `nums[k]` **两两不同** 。
    * 换句话说：`nums[i] != nums[j]`、`nums[i] != nums[k]` 且 `nums[j] != nums[k]` 。

返回满足上述条件三元组的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,4,2,4,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 下面列出的三元组均满足题目条件：
- (0, 2, 4) 因为 4 != 2 != 3
- (1, 2, 4) 因为 4 != 2 != 3
- (2, 3, 4) 因为 2 != 4 != 3
共计 3 个三元组，返回 3 。
注意 (2, 0, 4) 不是有效的三元组，因为 2 > 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在满足条件的三元组，所以返回 0 。
</pre>

#### 提示:
* `3 <= nums.length <= 100`
* `1 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in &nums {
            count.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }

        let count = count.values().collect::<Vec<_>>();

        for i in 0..count.len() {
            for j in i + 1..count.len() {
                for k in j + 1..count.len() {
                    ret += count[i] * count[j] * count[k];
                }
            }
        }

        ret
    }
}
```
