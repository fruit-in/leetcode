# 1785. 构成特定和需要添加的最少元素
给你一个整数数组 `nums` ，和两个整数 `limit` 与 `goal` 。数组 `nums` 有一条重要属性：`abs(nums[i]) <= limit` 。

返回使数组元素总和等于 `goal` 所需要向数组中添加的 **最少元素数量** ，添加元素 **不应改变** 数组中 `abs(nums[i]) <= limit` 这一属性。

注意，如果 `x >= 0` ，那么 `abs(x)` 等于 `x` ；否则，等于 `-x` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,-1,1], limit = 3, goal = -4
<strong>输出:</strong> 2
<strong>解释:</strong> 可以将 -2 和 -3 添加到数组中，数组的元素总和变为 1 - 1 + 1 - 2 - 3 = -4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,-10,9,1], limit = 100, goal = 0
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= limit <= 10<sup>6</sup></code>
* `-limit <= nums[i] <= limit`
* <code>-10<sup>9</sup> <= goal <= 10<sup>9</sup></code>

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} nums
# @param {Integer} limit
# @param {Integer} goal
# @return {Integer}
def min_elements(nums, limit, goal)
  ((nums.sum - goal).abs * 1.0 / limit).ceil
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        ((nums.iter().map(|&x| x as i64).sum::<i64>() - goal as i64).abs() as f64 / limit as f64)
            .ceil() as i32
    }
}
```
