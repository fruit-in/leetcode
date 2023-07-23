# 1827. 最少操作使数组递增
给你一个整数数组 `nums` （**下标从 0 开始**）。每一次操作中，你可以选择数组中一个元素，并将它增加 `1` 。
* 比方说，如果 `nums = [1,2,3]` ，你可以选择增加 `nums[1]` 得到 <code>nums = [1,<b>3</b>,3]</code> 。

请你返回使 `nums` **严格递增** 的 **最少** 操作次数。

我们称数组 `nums` 是 **严格递增的** ，当它满足对于所有的 `0 <= i < nums.length - 1` 都有 `nums[i] < nums[i+1]` 。一个长度为 `1` 的数组是严格递增的一种特殊情况。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以进行如下操作：
1) 增加 nums[2] ，数组变为 [1,1,<b>2</b>] 。
2) 增加 nums[1] ，数组变为 [1,<b>2</b>,2] 。
3) 增加 nums[2] ，数组变为 [1,2,<b>3</b>] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,5,2,4,1]
<strong>输出:</strong> 14
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [8]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 5000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_operations(nums)
  ret = 0

  (1...nums.size).each do |i|
    ret += [nums[i], nums[i - 1] + 1].max - nums[i]
    nums[i] = [nums[i], nums[i - 1] + 1].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += nums[i].max(nums[i - 1] + 1) - nums[i];
            nums[i] = nums[i].max(nums[i - 1] + 1);
        }

        ret
    }
}
```
