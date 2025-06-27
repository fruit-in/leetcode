# 2369. 检查数组是否存在有效划分
给你一个下标从 **0** 开始的整数数组 `nums` ，你必须将数组划分为一个或多个 **连续** 子数组。

如果获得的这些子数组中每个都能满足下述条件 **之一** ，则可以称其为数组的一种 **有效** 划分：
1. 子数组 **恰** 由 `2` 个相等元素组成，例如，子数组 `[2,2]` 。
2. 子数组 **恰** 由 `3` 个相等元素组成，例如，子数组 `[4,4,4]` 。
3. 子数组 **恰** 由 `3` 个连续递增元素组成，并且相邻元素之间的差值为 `1` 。例如，子数组 `[3,4,5]` ，但是子数组 `[1,3,5]` 不符合要求。

如果数组 **至少** 存在一种有效划分，返回 `true` ，否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,4,4,5,6]
<strong>输出:</strong> true
<strong>解释:</strong> 数组可以划分成子数组 [4,4] 和 [4,5,6] 。
这是一种有效划分，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,2]
<strong>输出:</strong> false
<strong>解释:</strong> 该数组不存在有效划分。
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def validPartition(self, nums: List[int]) -> bool:
        @cache
        def dfs(i: int) -> bool:
            return i == len(nums) or (i + 1 < len(nums) and nums[i] == nums[i + 1] and dfs(i + 2)) or (i + 2 < len(nums) and nums[i] == nums[i + 1] == nums[i + 2] and dfs(i + 3)) or (i + 2 < len(nums) and nums[i] == nums[i + 1] - 1 == nums[i + 2] - 2 and dfs(i + 3))

        return dfs(0)
```
