# 2449. 使数组相似的最少操作次数
给你两个正整数数组 `nums` 和 `target` ，两个数组长度相等。

在一次操作中，你可以选择两个 **不同** 的下标 `i` 和 `j` ，其中 `0 <= i, j < nums.length` ，并且：
* 令 `nums[i] = nums[i] + 2` 且
* 令 `nums[j] = nums[j] - 2` 。

如果两个数组中每个元素出现的频率相等，我们称两个数组是 **相似** 的。

请你返回将 `nums` 变得与 `target` 相似的最少操作次数。测试数据保证 `nums` 一定能变得与 `target` 相似。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [8,12,6], target = [2,14,10]
<strong>输出:</strong> 2
<strong>解释:</strong> 可以用两步操作将 nums 变得与 target 相似：
- 选择 i = 0 和 j = 2 ，nums = [10,12,4] 。
- 选择 i = 1 和 j = 2 ，nums = [10,14,2] 。
2 次操作是最少需要的操作次数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,5], target = [4,1,3]
<strong>输出:</strong> 1
<strong>解释:</strong> 一步操作可以使 nums 变得与 target 相似：
- 选择 i = 1 和 j = 2 ，nums = [1,4,3] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], target = [1,1,1,1,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组 nums 已经与 target 相似。
</pre>

#### 提示:
* `n == nums.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i], target[i] <= 10<sup>6</sup></code>
* `nums` 一定可以变得与 `target` 相似。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn make_similar(mut nums: Vec<i32>, mut target: Vec<i32>) -> i64 {
        nums.sort_unstable_by_key(|&x| (x % 2, x));
        target.sort_unstable_by_key(|&x| (x % 2, x));

        nums.iter()
            .zip(target.iter())
            .map(|(&x, &y)| (y - x).max(0) as i64 / 2)
            .sum()
    }
}
```
