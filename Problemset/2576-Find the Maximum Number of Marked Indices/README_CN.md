# 2576. 求出最多标记下标
给你一个下标从 **0** 开始的整数数组 `nums` 。

一开始，所有下标都没有被标记。你可以执行以下操作任意次：
* 选择两个 **互不相同且未标记** 的下标 `i` 和 `j` ，满足 `2 * nums[i] <= nums[j]` ，标记下标 `i` 和 `j` 。

请你执行上述操作任意次，返回 `nums` 中最多可以标记的下标数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,5,2,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 第一次操作中，选择 i = 2 和 j = 1 ，操作可以执行的原因是 2 * nums[2] <= nums[1] ，标记下标 2 和 1 。
没有其他更多可执行的操作，所以答案为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,2,5,4]
<strong>输出:</strong> 4
<strong>解释:</strong> 第一次操作中，选择 i = 3 和 j = 0 ，操作可以执行的原因是 2 * nums[3] <= nums[0] ，标记下标 3 和 0 。
第二次操作中，选择 i = 1 和 j = 2 ，操作可以执行的原因是 2 * nums[1] <= nums[2] ，标记下标 1 和 2 。
没有其他更多可执行的操作，所以答案为 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [7,6,8]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有任何可以执行的操作，所以答案为 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maxNumOfMarkedIndices(self, nums: List[int]) -> int:
        n = len(nums)
        lo = n // 2
        nums.sort()

        for i in range(n // 2):
            lo = bisect_left(nums, 2 * nums[i], lo=lo) + 1
            if lo > n:
                return i * 2

        return n // 2 * 2
```
