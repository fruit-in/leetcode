# 327. 区间和的个数
给你一个整数数组 `nums` 以及两个整数 `lower` 和 `upper` 。求数组中，值位于范围 `[lower, upper]` （包含 `lower` 和 `upper`）之内的 **区间和的个数** 。

**区间和** `S(i, j)` 表示在 `nums` 中，位置从 `i` 到 `j` 的元素之和，包含 `i` 和 `j` (`i` ≤ `j`)。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-2,5,-1], lower = -2, upper = 2
<strong>输出:</strong> 3
<strong>解释:</strong> 存在三个区间：[0,0]、[2,2] 和 [0,2] ，对应的区间和分别是：-2 、-1 、2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0], lower = 0, upper = 0
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* <code>-10<sup>5</sup> <= lower <= upper <= 10<sup>5</sup></code>
* 题目数据保证答案是一个 **32 位** 的整数

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        prefixsums = SortedList([0])
        prefixsum = 0
        ret = 0

        for i in range(len(nums)):
            prefixsum += nums[i]
            j = prefixsums.bisect_right(prefixsum - lower)
            k = prefixsums.bisect_left(prefixsum - upper)
            ret += j - k
            prefixsums.add(prefixsum)

        return ret
```
