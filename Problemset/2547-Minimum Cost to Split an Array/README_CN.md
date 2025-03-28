# 2547. 拆分数组的最小代价
给你一个整数数组 `nums` 和一个整数 `k` 。

将数组拆分成一些非空子数组。拆分的 **代价** 是每个子数组中的 **重要性** 之和。

令 `trimmed(subarray)` 作为子数组的一个特征，其中所有仅出现一次的数字将会被移除。

* 例如，`trimmed([3,1,2,4,3,4]) = [3,4,3,4]` 。

子数组的 **重要性** 定义为 `k + trimmed(subarray).length` 。

* 例如，如果一个子数组是 `[1,2,3,3,3,4,4]` ，`trimmed([1,2,3,3,3,4,4]) = [3,3,3,4,4]` 。这个子数组的重要性就是 `k + 5` 。

找出并返回拆分 `nums` 的所有可行方案中的最小代价。

**子数组** 是数组的一个连续 **非空** 元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,1,3,3], k = 2
<strong>输出:</strong> 8
<strong>解释:</strong> 将 nums 拆分成两个子数组：[1,2], [1,2,1,3,3]
[1,2] 的重要性是 2 + (0) = 2 。
[1,2,1,3,3] 的重要性是 2 + (2 + 2) = 6 。
拆分的代价是 2 + 6 = 8 ，可以证明这是所有可行的拆分方案中的最小代价。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,1], k = 2
<strong>输出:</strong> 6
<strong>解释:</strong> 将 nums 拆分成两个子数组：[1,2], [1,2,1] 。
[1,2] 的重要性是 2 + (0) = 2 。
[1,2,1] 的重要性是 2 + (2) = 4 。
拆分的代价是 2 + 4 = 6 ，可以证明这是所有可行的拆分方案中的最小代价。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,1], k = 5
<strong>输出:</strong> 10
<strong>解释:</strong> 将 nums 拆分成一个子数组：[1,2,1,2,1].
[1,2,1,2,1] 的重要性是 5 + (3 + 2) = 10 。
拆分的代价是 10 ，可以证明这是所有可行的拆分方案中的最小代价。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `0 <= nums[i] < nums.length`
* <code>1 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut dp = vec![i64::MAX; nums.len() + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            let mut count = HashMap::new();
            let mut trimmed_length = 0;

            for j in (0..i).rev() {
                *count.entry(nums[j]).or_insert(0) += 1;
                trimmed_length += match count.get(&nums[j]).unwrap() {
                    &1 => 0,
                    &2 => 2,
                    _ => 1,
                };
                dp[i] = dp[i].min(dp[j] + k + trimmed_length);
            }
        }

        *dp.last().unwrap() as i32
    }
}
```
