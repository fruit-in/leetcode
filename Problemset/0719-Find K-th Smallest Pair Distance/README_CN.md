# 719. 找出第 K 小的数对距离
数对 `(a,b)` 由整数 `a` 和 `b` 组成，其数对距离定义为 `a` 和 `b` 的绝对差值。

给你一个整数数组 `nums` 和一个整数 `k` ，数对由 `nums[i]` 和 `nums[j]` 组成且满足 `0 <= i < j < nums.length` 。返回 **所有数对距离中** 第 `k` 小的数对距离。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,1], k = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 数对和对应的距离如下：
(1,3) -> 2
(1,1) -> 0
(3,1) -> 2
距离第 1 小的数对是 (1,1) ，距离为 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1], k = 2
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,6,1], k = 3
<strong>输出:</strong> 5
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= n * (n - 1) / 2`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def smallestDistancePair(self, nums: List[int], k: int) -> int:
        nums.sort()

        lo = 0
        hi = nums[-1] - nums[0]

        while lo < hi:
            mid = (lo + hi) // 2
            count = 0

            for i in range(len(nums)):
                j = bisect.bisect(nums, nums[i] + mid, lo=i + 1)
                count += j - i - 1

            if count < k:
                lo = mid + 1
            else:
                hi = mid

        return hi
```
