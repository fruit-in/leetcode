# 2150. 找出数组中的所有孤独数字
给你一个整数数组 `nums` 。如果数字 `x` 在数组中仅出现 **一次** ，且没有 **相邻** 数字（即，`x + 1` 和 `x - 1`）出现在数组中，则认为数字 `x` 是 **孤独数字** 。

返回 `nums` 中的 **所有** 孤独数字。你可以按 **任何顺序** 返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,6,5,8]
<strong>输出:</strong> [10,8]
<strong>解释:</strong>
- 10 是一个孤独数字，因为它只出现一次，并且 9 和 11 没有在 nums 中出现。
- 8 是一个孤独数字，因为它只出现一次，并且 7 和 9 没有在 nums 中出现。
- 5 不是一个孤独数字，因为 6 出现在 nums 中，反之亦然。
因此，nums 中的孤独数字是 [10, 8] 。
注意，也可以返回 [8, 10] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,5,3]
<strong>输出:</strong> [1,5]
<strong>解释:</strong>
- 1 是一个孤独数字，因为它只出现一次，并且 0 和 2 没有在 nums 中出现。
- 5 是一个孤独数字，因为它只出现一次，并且 4 和 6 没有在 nums 中出现。
- 3 不是一个孤独数字，因为它出现两次。
因此，nums 中的孤独数字是 [1, 5] 。
注意，也可以返回 [5, 1] 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut is_lonely = HashMap::new();

        for &n in &nums {
            is_lonely
                .entry(n)
                .and_modify(|b| *b = false)
                .or_insert(true);
            is_lonely.insert(n + 1, false);
            is_lonely.insert(n - 1, false);
        }

        nums.into_iter().filter(|n| is_lonely[n]).collect()
    }
}
```
