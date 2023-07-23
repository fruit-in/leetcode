# 1984. 学生分数的最小差值
给你一个 **下标从 0 开始** 的整数数组 `nums` ，其中 `nums[i]` 表示第 `i` 名学生的分数。另给你一个整数 `k` 。

从数组中选出任意 `k` 名学生的分数，使这 `k` 个分数间 **最高分** 和 **最低分** 的 **差值** 达到 **最小化** 。

返回可能的 **最小差值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [90], k = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 选出 1 名学生的分数，仅有 1 种方法：
- [90] 最高分和最低分之间的差值是 90 - 90 = 0
可能的最小差值是 0
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,4,1,7], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 选出 2 名学生的分数，有 6 种方法：
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 4 = 5
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 1 = 8
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 7 = 2
- [9,4,1,7] 最高分和最低分之间的差值是 4 - 1 = 3
- [9,4,1,7] 最高分和最低分之间的差值是 7 - 4 = 3
- [9,4,1,7] 最高分和最低分之间的差值是 7 - 1 = 6
可能的最小差值是 2
</pre>

#### 提示:
* `1 <= k <= nums.length <= 1000`
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();

        (0..=nums.len() - k)
            .map(|i| nums[i + k - 1] - nums[i])
            .min()
            .unwrap()
    }
}
```
