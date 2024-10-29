# 2025. 分割数组的最多方案数
给你一个下标从 **0** 开始且长度为 `n` 的整数数组 `nums` 。**分割** 数组 `nums` 的方案数定义为符合以下两个条件的 `pivot` 数目：

* `1 <= pivot < n`
* `nums[0] + nums[1] + ... + nums[pivot - 1] == nums[pivot] + nums[pivot + 1] + ... + nums[n - 1]`

同时给你一个整数 `k` 。你可以将 `nums` 中 **一个** 元素变为 `k` 或 **不改变** 数组。

请你返回在 **至多** 改变一个元素的前提下，**最多** 有多少种方法 **分割** `nums` 使得上述两个条件都满足。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,-1,2], k = 3
<strong>输出:</strong> 1
<strong>解释:</strong> 一个最优的方案是将 nums[0] 改为 k 。数组变为 [3,-1,2] 。
有一种方法分割数组：
- pivot = 2 ，我们有分割 [3,-1 | 2]：3 + -1 == 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,0], k = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 一个最优的方案是不改动数组。
有两种方法分割数组：
- pivot = 1 ，我们有分割 [0 | 0,0]：0 == 0 + 0 。
- pivot = 2 ，我们有分割 [0,0 | 0]: 0 + 0 == 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [22,4,-25,-20,-15,15,-16,7,19,-10,0,-13,-14], k = -33
<strong>输出:</strong> 4
<strong>解释:</strong> 一个最优的方案是将 nums[2] 改为 k 。数组变为 [22,4,-33,-20,-15,15,-16,7,19,-10,0,-13,-14] 。
有四种方法分割数组。
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= k, nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as i64;
        let mut sum = nums[0] as i64;
        let mut prefix_sum = nums[0] as i64;
        let mut suffix_sum = 0;
        let mut prefix_count = HashMap::from([(prefix_sum, 1)]);
        let mut suffix_count = HashMap::new();
        let mut ret = 0;

        for i in (1..n).rev() {
            sum += nums[i] as i64;
            suffix_sum += nums[i] as i64;
            *suffix_count.entry(suffix_sum).or_insert(0) += 1;
        }

        if sum % 2 == 0 {
            ret = *suffix_count.get(&(sum / 2)).unwrap_or(&0);
        }

        if (suffix_sum + k) % 2 == 0 {
            ret = ret.max(*suffix_count.get(&((suffix_sum + k) / 2)).unwrap_or(&0));
        }

        for i in 1..n {
            let new_sum = sum - nums[i] as i64 + k;

            *suffix_count.get_mut(&suffix_sum).unwrap() -= 1;
            suffix_sum -= nums[i] as i64;

            if new_sum % 2 == 0 {
                let x = *prefix_count.get(&(new_sum / 2)).unwrap_or(&0);
                let y = *suffix_count.get(&(new_sum / 2)).unwrap_or(&0);
                ret = ret.max(x + y);
            }

            prefix_sum += nums[i] as i64;
            *prefix_count.entry(prefix_sum).or_insert(0) += 1;
        }

        ret
    }
}
```
