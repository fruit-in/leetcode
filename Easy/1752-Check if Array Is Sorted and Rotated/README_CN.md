# 1752. 检查数组是否经排序和轮转得到
给你一个数组 `nums` 。`nums` 的源数组中，所有元素与 `nums` 相同，但按非递减顺序排列。

如果 `nums` 能够由源数组轮转若干位置（包括 0 个位置）得到，则返回 `true` ；否则，返回 `false` 。

源数组中可能存在 **重复项** 。

**注意:** 我们称数组 `A` 在轮转 `x` 个位置后得到长度相同的数组 `B` ，当它们满足 `A[i] == B[(i+x) % A.length]` ，其中 `%` 为取余运算。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,5,1,2]
<strong>输出:</strong> true
<strong>解释:</strong> [1,2,3,4,5] 为有序的源数组。
可以轮转 x = 3 个位置，使新数组从值为 3 的元素开始：[3,4,5,1,2] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,1,3,4]
<strong>输出:</strong> false
<strong>解释:</strong> 源数组无法经轮转得到 nums 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> true
<strong>解释:</strong> [1,2,3] 为有序的源数组。
可以轮转 x = 0 个位置（即不轮转）得到 nums 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> true
<strong>解释:</strong> [1,1,1] 为有序的源数组。
轮转任意个位置都可以得到 nums 。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> nums = [2,1]
<strong>输出:</strong> true
<strong>解释:</strong> [1,2] 为有序的源数组。
可以轮转 x = 5 个位置，使新数组从值为 2 的元素开始：[2,1] 。
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def check(nums)
  count = (1...nums.size).count { |i| nums[i] < nums[i - 1] }
  count += 1 if nums[0] < nums[-1]

  count < 2
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        match (1..nums.len()).filter(|&i| nums[i] < nums[i - 1]).count() {
            0 => true,
            1 => nums[0] >= nums[nums.len() - 1],
            _ => false,
        }
    }
}
```
