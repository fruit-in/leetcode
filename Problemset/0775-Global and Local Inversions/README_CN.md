# 775. 全局倒置与局部倒置
给你一个长度为 `n` 的整数数组 `nums` ，表示由范围 `[0, n - 1]` 内所有整数组成的一个排列。

**全局倒置** 的数目等于满足下述条件不同下标对 `(i, j)` 的数目：
* `0 <= i < j < n`
* `nums[i] > nums[j]`

**局部倒置** 的数目等于满足下述条件的下标 `i` 的数目：
* `0 <= i < n - 1`
* `nums[i] > nums[i + 1]`

当数组 `nums` 中 **全局倒置** 的数量等于 **局部倒置** 的数量时，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,0,2]
<strong>输出:</strong> true
<strong>解释:</strong> 有 1 个全局倒置，和 1 个局部倒置。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,0]
<strong>输出:</strong> false
<strong>解释:</strong> 有 2 个全局倒置，和 1 个局部倒置。
</pre>

#### 提示:
* `n == nums.length`
* `1 <= n <= 5000`
* `0 <= nums[i] < n`
* `nums` 中的所有整数 **互不相同**
* `nums` 是范围 `[0, n - 1]` 内所有数字组成的一个排列

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def is_ideal_permutation(nums)
  max = 0

  (2...nums.size).each do |i|
    max = [max, nums[i - 2]].max
    return false if nums[i] < max
  end

  true
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut max = 0;

        for i in 2..nums.len() {
            max = max.max(nums[i - 2]);
            if nums[i] < max {
                return false;
            }
        }

        true
    }
}
```
