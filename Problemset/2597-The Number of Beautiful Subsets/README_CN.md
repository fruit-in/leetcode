# 2597. 美丽子集的数目
给你一个由正整数组成的数组 `nums` 和一个 **正** 整数 `k` 。

如果 `nums` 的子集中，任意两个整数的绝对差均不等于 `k` ，则认为该子数组是一个 **美丽** 子集。

返回数组 `nums` 中 **非空** 且 **美丽** 的子集数目。

`nums` 的子集定义为：可以经由 `nums` 删除某些元素（也可能不删除）得到的一个数组。只有在删除元素时选择的索引不同的情况下，两个子集才会被视作是不同的子集。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,6], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 数组 nums 中的美丽子集有：[2], [4], [6], [2, 6] 。
可以证明数组 [2,4,6] 中只存在 4 个美丽子集。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1], k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 数组 nums 中的美丽数组有：[1] 。
可以证明数组 [1] 中只存在 1 个美丽子集。
</pre>

#### 提示:
* `1 <= nums.length <= 18`
* `1 <= nums[i], k <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(nums):
                return

            j = bisect.bisect_left(subset, nums[i] - k)
            if j == len(subset) or subset[j] != nums[i] - k:
                subset.append(nums[i])
                ret += 1
                dfs(i + 1)
                subset.pop()
            dfs(i + 1)

        subset = []
        ret = 0
        nums.sort()

        dfs(0)

        return ret
```
