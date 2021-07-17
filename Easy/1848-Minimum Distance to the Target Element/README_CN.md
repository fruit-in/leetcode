# 1848. 到目标元素的最小距离
给你一个整数数组 `nums` （下标 **从 0 开始** 计数）以及两个整数 `target` 和 `start` ，请你找出一个下标 `i` ，满足 `nums[i] == target` 且 `abs(i - start)` **最小化** 。注意：`abs(x)` 表示 `x` 的绝对值。

返回 `abs(i - start)` 。

题目数据保证 `target` 存在于 `nums` 中。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5], target = 5, start = 3
<strong>输出:</strong> 1
<strong>解释:</strong> nums[4] = 5 是唯一一个等于 target 的值，所以答案是 abs(4 - 3) = 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1], target = 1, start = 0
<strong>输出:</strong> 0
<strong>解释:</strong> nums[0] = 1 是唯一一个等于 target 的值，所以答案是 abs(0 - 0) = 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1,1,1,1,1,1], target = 1, start = 0
<strong>输出:</strong> 0
<strong>解释:</strong> nums 中的每个值都是 1 ，但 nums[0] 使 abs(i - start) 的结果得以最小化，所以答案是 abs(0 - 0) = 0 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `0 <= start < nums.length`
* `target` 存在于 `nums` 中

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let start = start as usize;

        for i in 0..nums.len() {
            if nums[(start + i).min(nums.len() - 1)] == target
                || nums[start.saturating_sub(i)] == target
            {
                return i as i32;
            }
        }

        unreachable!()
    }
}
```
