# 377. 组合总和 Ⅳ
给你一个由 **不同** 整数组成的数组 `nums` ，和一个目标整数 `target` 。请你从 `nums` 中找出并返回总和为 `target` 的元素组合的个数。

题目数据保证答案符合 32 位整数范围。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3], target = 4
<strong>输出:</strong> 7
<strong>解释:</strong>
所有可能的组合为：
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)
请注意，顺序不同的序列被视作不同的组合。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9], target = 3
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 1000`
* `nums` 中的所有元素 **互不相同**
* `1 <= target <= 1000`

**进阶：**如果给定的数组中含有负数会发生什么？问题会产生何种变化？如果允许负数出现，需要向题目中添加哪些限制条件？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;

        for i in 0..=target {
            for &x in &nums {
                if i >= x {
                    dp[i as usize] += dp[(i - x) as usize];
                }
            }
        }

        dp[target as usize]
    }
}
```
