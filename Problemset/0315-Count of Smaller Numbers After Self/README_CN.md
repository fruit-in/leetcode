# 315. 计算右侧小于当前元素的个数
给你一个整数数组 `nums` ，按要求返回一个新数组 `counts` 。数组 `counts` 有该性质： `counts[i]` 的值是  `nums[i]` 右侧小于 `nums[i]` 的元素的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,2,6,1]
<strong>输出:</strong> [2,1,1,0]
<strong>解释:</strong>
5 的右侧有 2 个更小的元素 (2 和 1)
2 的右侧仅有 1 个更小的元素 (1)
6 的右侧有 1 个更小的元素 (1)
1 的右侧有 0 个更小的元素
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1]
<strong>输出:</strong> [0]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-1,-1]
<strong>输出:</strong> [0,0]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        sortednums = SortedList()
        counts = [0] * len(nums)

        for i in range(len(nums) - 1, -1, -1):
            counts[i] = sortednums.bisect_left(nums[i])
            sortednums.add(nums[i])

        return counts
```
