# 698. 划分为k个相等的子集
给定一个整数数组  `nums` 和一个正整数 `k`，找出是否有可能把这个数组分成 `k` 个非空子集，其总和都相等。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,3,2,3,5,2,1], k = 4
<strong>输出:</strong> true
<strong>说明:</strong> 有可能将其分成 4 个子集（5），（1,4），（2,3），（2,3）等于总和。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 3
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= k <= nums.length <= 16`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* 每个元素的频率在 `[1,4]` 范围内

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        def dfs(i: int) -> bool:
            if i == len(nums):
                return True

            prevsets = set()

            for j in range(k):
                if subsets[j] not in prevsets and subsets[j] + nums[i] <= subsum:
                    prevsets.add(subsets[j])
                    subsets[j] += nums[i]
                    if dfs(i + 1):
                        return True
                    subsets[j] -= nums[i]

            return False

        if sum(nums) % k != 0:
            return False

        nums.sort()
        subsum = sum(nums) // k

        while nums and nums[-1] == subsum:
            nums.pop()
            k -= 1

        if k > 0 and (nums[-1] > subsum or nums[-1] + nums[0] > subsum):
            return False

        subsets = [0] * k
        nums.reverse()

        return dfs(0)
```
