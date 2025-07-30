# 1681. 最小不兼容性
给你一个整数数组 `nums` 和一个整数 `k` 。你需要将这个数组划分到 `k` 个相同大小的子集中，使得同一个子集里面没有两个相同的元素。

一个子集的 **不兼容性** 是该子集里面最大值和最小值的差。

请你返回将数组分成 `k` 个子集后，各子集 **不兼容性** 的 **和** 的 **最小值** ，如果无法分成分成 `k` 个子集，返回 `-1` 。

子集的定义是数组中一些数字的集合，对数字顺序没有要求。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1,4], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 最优的分配是 [1,2] 和 [1,4] 。
不兼容性和为 (2-1) + (4-1) = 4 。
注意到 [1,1] 和 [2,4] 可以得到更小的和，但是第一个集合有 2 个相同的元素，所以不可行。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [6,3,8,1,3,1,2,2], k = 4
<strong>输出:</strong> 6
<strong>解释:</strong> 最优的子集分配为 [1,2]，[2,3]，[6,8] 和 [1,3] 。
不兼容性和为 (2-1) + (3-2) + (8-6) + (3-1) = 6 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [5,3,3,6,3,3], k = 3
<strong>输出:</strong> -1
<strong>解释:</strong> 没办法将这些数字分配到 3 个子集且满足每个子集里没有相同数字。
</pre>

#### 提示:
* `1 <= k <= nums.length <= 16`
* `nums.length` 能被 `k` 整除。
* `1 <= nums[i] <= nums.length`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimumIncompatibility(self, nums: List[int], k: int) -> int:
        if max(Counter(nums).values()) > k:
            return -1
        if k == len(nums):
            return 0
        if k == 1:
            return max(nums) - min(nums)

        def dfs(i: int) -> None:
            nonlocal incompatibilities, minincompatibilities

            if subsets[-1] != [] and incompatibilities >= minincompatibilities:
                return

            if i == len(nums):
                minincompatibilities = incompatibilities
                return

            for j in range(k):
                if len(subsets[j]) == size or (subsets[j] != [] and subsets[j][-1] == nums[i]) or (j > 0 and subsets[j] == subsets[j - 1]):
                    continue

                subsets[j].append(nums[i])
                if len(subsets[j]) == 1:
                    incompatibilities -= nums[i]
                elif len(subsets[j]) == size:
                    incompatibilities += nums[i]
                dfs(i + 1)
                subsets[j].pop()
                if len(subsets[j]) == 0:
                    incompatibilities += nums[i]
                elif len(subsets[j]) == size - 1:
                    incompatibilities -= nums[i]

        size = len(nums) // k
        subsets = [[] for _ in range(k)]
        incompatibilities = 0
        minincompatibilities = inf
        nums.sort()
        dfs(0)

        return minincompatibilities
```
