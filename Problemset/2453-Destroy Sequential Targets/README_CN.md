# 2453. 摧毁一系列目标
给你一个下标从 **0** 开始的数组 `nums` ，它包含若干正整数，表示数轴上你需要摧毁的目标所在的位置。同时给你一个整数 `space` 。

你有一台机器可以摧毁目标。给机器 **输入** `nums[i]` ，这台机器会摧毁所有位置在 `nums[i] + c * space` 的目标，其中 `c` 是任意非负整数。你想摧毁 `nums` 中 **尽可能多** 的目标。

请你返回在摧毁数目最多的前提下，`nums[i]` 的 **最小值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,7,8,1,1,5], space = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 如果我们输入 nums[3] ，我们可以摧毁位于 1,3,5,7,9,... 这些位置的目标。
这种情况下， 我们总共可以摧毁 5 个目标（除了 nums[2]）。
没有办法摧毁多于 5 个目标，所以我们返回 nums[3] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,5,2,4,6], space = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 输入 nums[0] 或者 nums[3] 都会摧毁 3 个目标。
没有办法摧毁多于 3 个目标。
由于 nums[0] 是最小的可以摧毁 3 个目标的整数，所以我们返回 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [6,2,5], space = 100
<strong>输出:</strong> 2
<strong>解释:</strong> 无论我们输入哪个数字，都只能摧毁 1 个目标。输入的最小整数是 nums[1] 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= space <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut max_count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            match map.get_mut(&(nums[i] % space)) {
                Some((min_num, count)) => {
                    *min_num = nums[i].min(*min_num);
                    *count += 1;
                    if max_count < *count || (max_count == *count && ret > *min_num) {
                        max_count = *count;
                        ret = *min_num;
                    }
                }
                None if max_count == 0 || (max_count == 1 && ret > nums[i]) => {
                    max_count = 1;
                    ret = nums[i];
                    map.insert(nums[i] % space, (nums[i], 1));
                }
                None => {
                    map.insert(nums[i] % space, (nums[i], 1));
                }
            }
        }

        ret
    }
}
```
