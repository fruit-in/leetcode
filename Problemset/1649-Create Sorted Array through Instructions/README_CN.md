# 1649. 通过指令创建有序数组
给你一个整数数组 `instructions` ，你需要根据 `instructions` 中的元素创建一个有序数组。一开始你有一个空的数组 `nums` ，你需要 **从左到右** 遍历 `instructions` 中的元素，将它们依次插入 `nums` 数组中。每一次插入操作的 **代价** 是以下两者的 **较小值** ：

* `nums` 中 **严格小于**  `instructions[i]` 的数字数目。
* `nums` 中 **严格大于**  `instructions[i]` 的数字数目。

比方说，如果要将 `3` 插入到 `nums = [1,2,3,5]` ，那么插入操作的 **代价** 为 `min(2, 1)` (元素 `1` 和  `2` 小于 `3` ，元素 `5` 大于 `3` ），插入后 `nums` 变成 `[1,2,3,3,5]` 。

请你返回将 `instructions` 中所有元素依次插入 `nums` 后的 **总最小代价** 。由于答案会很大，请将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> instructions = [1,5,6,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 一开始 nums = [] 。
插入 1 ，代价为 min(0, 0) = 0 ，现在 nums = [1] 。
插入 5 ，代价为 min(1, 0) = 0 ，现在 nums = [1,5] 。
插入 6 ，代价为 min(2, 0) = 0 ，现在 nums = [1,5,6] 。
插入 2 ，代价为 min(1, 2) = 1 ，现在 nums = [1,2,5,6] 。
总代价为 0 + 0 + 0 + 1 = 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> instructions = [1,2,3,6,5,4]
<strong>输出:</strong> 3
<strong>解释:</strong> 一开始 nums = [] 。
插入 1 ，代价为 min(0, 0) = 0 ，现在 nums = [1] 。
插入 2 ，代价为 min(1, 0) = 0 ，现在 nums = [1,2] 。
插入 3 ，代价为 min(2, 0) = 0 ，现在 nums = [1,2,3] 。
插入 6 ，代价为 min(3, 0) = 0 ，现在 nums = [1,2,3,6] 。
插入 5 ，代价为 min(3, 1) = 1 ，现在 nums = [1,2,3,5,6] 。
插入 4 ，代价为 min(3, 2) = 2 ，现在 nums = [1,2,3,4,5,6] 。
总代价为 0 + 0 + 0 + 0 + 1 + 2 = 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> instructions = [1,3,3,3,2,4,2,1,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 一开始 nums = [] 。
插入 1 ，代价为 min(0, 0) = 0 ，现在 nums = [1] 。
插入 3 ，代价为 min(1, 0) = 0 ，现在 nums = [1,3] 。
插入 3 ，代价为 min(1, 0) = 0 ，现在 nums = [1,3,3] 。
插入 3 ，代价为 min(1, 0) = 0 ，现在 nums = [1,3,3,3] 。
插入 2 ，代价为 min(1, 3) = 1 ，现在 nums = [1,2,3,3,3] 。
插入 4 ，代价为 min(5, 0) = 0 ，现在 nums = [1,2,3,3,3,4] 。
插入 2 ，代价为 min(1, 4) = 1 ，现在 nums = [1,2,2,3,3,3,4] 。
插入 1 ，代价为 min(0, 6) = 0 ，现在 nums = [1,1,2,2,3,3,3,4] 。
插入 2 ，代价为 min(2, 4) = 2 ，现在 nums = [1,1,2,2,2,3,3,3,4] 。
总代价为 0 + 0 + 0 + 0 + 1 + 0 + 1 + 0 + 2 = 4 。
</pre>

#### 提示:
* <code>1 <= instructions.length <= 10<sup>5</sup></code>
* <code>1 <= instructions[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def createSortedArray(self, instructions: List[int]) -> int:
        nums = SortedList()
        ret = 0

        for num in instructions:
            nums.add(num)
            less = nums.bisect_left(num)
            greater = len(nums) - nums.bisect_right(num)
            ret += min(less, greater)

        return ret % 1000000007
```
