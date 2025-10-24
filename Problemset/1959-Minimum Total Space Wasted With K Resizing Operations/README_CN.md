# 1959. K 次调整数组大小浪费的最小总空间
你正在设计一个动态数组。给你一个下标从 **0** 开始的整数数组 `nums` ，其中 `nums[i]` 是 `i` 时刻数组中的元素数目。除此以外，你还有一个整数 `k` ，表示你可以 **调整** 数组大小的 **最多** 次数（每次都可以调整成 **任意** 大小）。

`t` 时刻数组的大小 <code>size<sub>t</sub></code> 必须大于等于 `nums[t]` ，因为数组需要有足够的空间容纳所有元素。`t` 时刻 **浪费的空间** 为 <code>size<sub>t</sub> - nums[t]</code> ，**总** 浪费空间为满足 `0 <= t < nums.length` 的每一个时刻 `t` 浪费的空间 **之和** 。

在调整数组大小不超过 `k` 次的前提下，请你返回 **最小总浪费空间** 。

**注意：**数组最开始时可以为 **任意大小** ，且 **不计入** 调整大小的操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,20], k = 0
<strong>输出:</strong> 10
<strong>解释:</strong> size = [20,20].
我们可以让数组初始大小为 20 。
总浪费空间为 (20 - 10) + (20 - 20) = 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,20,30], k = 1
<strong>输出:</strong> 10
<strong>解释:</strong> size = [20,20,30].
我们可以让数组初始大小为 20 ，然后时刻 2 调整大小为 30 。
总浪费空间为 (20 - 10) + (20 - 20) + (30 - 30) = 10 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [10,20,15,30,20], k = 2
<strong>输出:</strong> 15
<strong>解释:</strong> size = [10,20,20,30,30].
我们可以让数组初始大小为 10 ，时刻 1 调整大小为 20 ，时刻 3 调整大小为 30 。
总浪费空间为 (10 - 10) + (20 - 20) + (20 - 15) + (30 - 30) + (30 - 20) = 15 。
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* `0 <= k <= nums.length - 1`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minSpaceWastedKResizing(self, nums: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(nums):
                return 0
            if k == 1:
                return max(nums[i:]) * (len(nums) - i) - sum(nums[i:])

            maxnum = 0
            numssum = 0
            ret = inf

            for j in range(i, len(nums)):
                maxnum = max(maxnum, nums[j])
                numssum += nums[j]
                ret = min(ret, maxnum * (j - i + 1) -
                          numssum + dfs(j + 1, k - 1))

            return ret

        return dfs(0, k + 1)
```
