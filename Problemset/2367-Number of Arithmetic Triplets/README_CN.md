# 2367. 算术三元组的数目
给你一个下标从 **0** 开始、**严格递增** 的整数数组 `nums` 和一个正整数 `diff` 。如果满足下述全部条件，则三元组 `(i, j, k)` 就是一个 **算术三元组** ：

* `i < j < k` ，
* `nums[j] - nums[i] == diff` 且
* `nums[k] - nums[j] == diff`

返回不同 **算术三元组** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,4,6,7,10], diff = 3
<strong>输出:</strong> 2
<strong>解释:</strong>
(1, 2, 4) 是算术三元组：7 - 4 == 3 且 4 - 1 == 3 。
(2, 4, 5) 是算术三元组：10 - 7 == 3 且 7 - 4 == 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,5,6,7,8,9], diff = 2
<strong>输出:</strong> 2
<strong>解释:</strong>
(0, 2, 4) 是算术三元组：8 - 6 == 2 且 6 - 4 == 2 。
(1, 3, 5) 是算术三元组：9 - 7 == 2 且 7 - 5 == 2 。
</pre>

#### 提示:
* `3 <= nums.length <= 200`
* `0 <= nums[i] <= 200`
* `1 <= diff <= 50`
* `nums` **严格** 递增

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut nums_set = HashSet::from([nums[0], nums[1]]);
        let mut ret = 0;

        for &num in &nums[2..] {
            nums_set.insert(num);

            if nums_set.contains(&(num - diff)) && nums_set.contains(&(num - 2 * diff)) {
                ret += 1;
            }
        }

        ret
    }
}
```
