# 493. 翻转对
给定一个数组 `nums` ，如果 `i < j` 且 `nums[i] > 2*nums[j]` 我们就将 `(i, j)` 称作一个***重要翻转对***。

你需要返回给定数组中的重要翻转对的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,2,3,1]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,4,3,5,1]
<strong>输出:</strong> 3
</pre>

#### 注意:
1. 给定数组的长度不会超过`50000`。
2. 输入数组中的所有数字都在32位整数的表示范围内。

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def reversePairs(self, nums: List[int]) -> int:
        sortednums = SortedList()
        ret = 0

        for num in nums:
            ret += len(sortednums) - sortednums.bisect_right(2 * num)
            sortednums.add(num)

        return ret
```
