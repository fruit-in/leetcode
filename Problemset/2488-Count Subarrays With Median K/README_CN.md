# 2488. 统计中位数为 K 的子数组
给你一个长度为 `n` 的数组 `nums` ，该数组由从 `1` 到 `n` 的 **不同** 整数组成。另给你一个正整数 `k` 。

统计并返回 `nums` 中的 **中位数** 等于 `k` 的非空子数组的数目。

#### 注意：

* 数组的中位数是按 **递增** 顺序排列后位于 **中间** 的那个元素，如果数组长度为偶数，则中位数是位于中间靠 **左** 的那个元素。
    * 例如，`[2,3,1,4]` 的中位数是 `2` ，`[8,4,3,5,1]` 的中位数是 `4` 。
* 子数组是数组中的一个连续部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,2,1,4,5], k = 4
<strong>输出:</strong> 3
<strong>解释:</strong> 中位数等于 4 的子数组有：[4]、[4,5] 和 [1,4,5] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,1], k = 3
<strong>输出:</strong> 1
<strong>解释:</strong> [3] 是唯一一个中位数等于 3 的子数组。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= nums[i], k <= n`
* `nums` 中的整数互不相同

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        if !nums.contains(&k) {
            return 0;
        }

        let i = nums.iter().position(|&x| x == k).unwrap();
        let mut diff = 0;
        let mut count = HashMap::from([(0, 1)]);
        let mut ret = 1;

        for j in i + 1..nums.len() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            if diff == 0 || diff == 1 {
                ret += 1;
            }
            *count.entry(diff).or_insert(0) += 1;
        }

        diff = 0;

        for j in (0..i).rev() {
            if nums[j] > k {
                diff += 1;
            } else {
                diff -= 1;
            }
            ret += count.get(&-diff).unwrap_or(&0);
            ret += count.get(&(1 - diff)).unwrap_or(&0);
        }

        ret
    }
}
```
