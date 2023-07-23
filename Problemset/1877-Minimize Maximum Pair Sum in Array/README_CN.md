# 1877. 数组中最大数对和的最小值
一个数对 `(a,b)` 的 **数对和** 等于 `a + b` 。**最大数对和** 是一个数对数组中最大的 **数对和** 。
* 比方说，如果我们有数对 `(1,5)` ，`(2,3)` 和 `(4,4)`，**最大数对和** 为 `max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8` 。

给你一个长度为 **偶数** `n` 的数组 `nums` ，请你将 `nums` 中的元素分成 `n / 2` 个数对，使得：
* `nums` 中每个元素 **恰好** 在 一个 数对中，且
* **最大数对和** 的值 **最小** 。

请你在最优数对划分的方案下，返回最小的 **最大数对和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,5,2,3]
<strong>输出:</strong> 7
<strong>解释:</strong> 数组中的元素可以分为数对 (3,3) 和 (5,2) 。
最大数对和为 max(3+3, 5+2) = max(6, 7) = 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,5,4,2,4,6]
<strong>输出:</strong> 8
<strong>解释:</strong> 数组中的元素可以分为数对 (3,5)，(4,4) 和 (6,2) 。
最大数对和为 max(3+5, 4+4, 6+2) = max(8, 8, 8) = 8 。
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `n` 是 **偶数** 。
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        (0..nums.len() / 2)
            .map(|i| nums[i] + nums[nums.len() - 1 - i])
            .max()
            .unwrap()
    }
}
```
