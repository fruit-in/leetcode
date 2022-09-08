# 2270. 分割数组的方案数
给你一个下标从 **0** 开始长度为 `n` 的整数数组 `nums` 。
如果以下描述为真，那么 `nums` 在下标 `i` 处有一个 **合法的分割** ：
* 前 `i + 1` 个元素的和 **大于等于** 剩下的 `n - i - 1` 个元素的和。
* 下标 `i` 的右边 **至少有一个** 元素，也就是说下标 `i` 满足 `0 <= i < n - 1` 。

请你返回 `nums` 中的 **合法分割** 方案数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,4,-8,7]
<strong>输出:</strong> 2
<strong>解释:</strong>
总共有 3 种不同的方案可以将 nums 分割成两个非空的部分：
- 在下标 0 处分割 nums 。那么第一部分为 [10] ，和为 10 。第二部分为 [4,-8,7] ，和为 3 。因为 10 >= 3 ，所以 i = 0 是一个合法的分割。
- 在下标 1 处分割 nums 。那么第一部分为 [10,4] ，和为 14 。第二部分为 [-8,7] ，和为 -1 。因为 14 >= -1 ，所以 i = 1 是一个合法的分割。
- 在下标 2 处分割 nums 。那么第一部分为 [10,4,-8] ，和为 6 。第二部分为 [7] ，和为 7 。因为 6 < 7 ，所以 i = 2 不是一个合法的分割。
所以 nums 中总共合法分割方案受为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,1,0]
<strong>输出:</strong> 2
<strong>解释:</strong>
总共有 2 种 nums 的合法分割：
- 在下标 1 处分割 nums 。那么第一部分为 [2,3] ，和为 5 。第二部分为 [1,0] ，和为 1 。因为 5 >= 1 ，所以 i = 1 是一个合法的分割。
- 在下标 2 处分割 nums 。那么第一部分为 [2,3,1] ，和为 6 。第二部分为 [0] ，和为 0 。因为 6 >= 0 ，所以 i = 2 是一个合法的分割。
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut sum_l = 0;
        let mut sum_r = nums.iter().map(|&x| x as i64).sum();
        let mut ret = 0;

        for i in 0..nums.len() - 1 {
            sum_l += nums[i] as i64;
            sum_r -= nums[i] as i64;

            if sum_l >= sum_r {
                ret += 1;
            }
        }

        ret
    }
}
```
