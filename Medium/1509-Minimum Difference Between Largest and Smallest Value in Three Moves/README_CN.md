# 1509. 三次操作后最大值与最小值的最小差
给你一个数组 `nums` ，每次操作你可以选择 `nums` 中的任意一个元素并将它改成任意值。

请你返回三次操作后， `nums` 中最大值与最小值的差的最小值。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,3,2,4]
<strong>输出:</strong> 0
<strong>解释:</strong> 将数组 [5,3,2,4] 变成 [2,2,2,2].
最大值与最小值的差为 2-2 = 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,5,0,10,14]
<strong>输出:</strong> 1
<strong>解释:</strong> 将数组 [1,5,0,10,14] 变成 [1,1,0,1,1] 。
最大值与最小值的差为 1-0 = 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [6,6,0,1,1,4,6]
<strong>输出:</strong> 2
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,5,6,14,15]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `-10^9 <= nums[i] <= 10^9`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_difference(nums)
  return 0 if nums.length < 5

  maxs = nums.max(4)
  mins = nums.min(4)

  [maxs[0] - mins[3], maxs[1] - mins[2], maxs[2] - mins[1], maxs[3] - mins[0]].min
end
```
