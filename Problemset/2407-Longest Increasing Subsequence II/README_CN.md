# 2407. 最长递增子序列 II
给你一个整数数组 `nums` 和一个整数 `k` 。

找到 `nums` 中满足以下要求的最长子序列：
* 子序列 **严格递增**
* 子序列中相邻元素的差值 **不超过** `k` 。

请你返回满足上述要求的 **最长子序列** 的长度。

**子序列** 是从一个数组中删除部分元素后，剩余元素不改变顺序得到的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,2,1,4,3,4,5,8,15], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong>
满足要求的最长子序列是 [1,3,4,5,8] 。
子序列长度为 5 ，所以我们返回 5 。
注意子序列 [1,3,4,5,8,15] 不满足要求，因为 15 - 8 = 7 大于 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [7,4,5,1,8,12,4,7], k = 5
<strong>输出:</strong> 4
<strong>解释:</strong>
满足要求的最长子序列是 [4,5,8,12] 。
子序列长度为 4 ，所以我们返回 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,5], k = 1
<strong>输出:</strong> 1
<strong>解释:</strong>
满足要求的最长子序列是 [1] 。
子序列长度为 1 ，所以我们返回 1 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], k <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def lengthOfLIS(self, nums: List[int], k: int) -> int:
        size = 1 << ceil(log2(max(nums) + 1))
        tree = [0] * (2 * size)
        dp = [0] * len(nums)

        for i in range(len(nums)):
            left = max(nums[i] - k, 0) + size
            right = nums[i] - 1 + size
            while left <= right:
                if left % 2 == 1:
                    dp[i] = max(dp[i], tree[left] + 1)
                    left += 1
                if right % 2 == 0:
                    dp[i] = max(dp[i], tree[right] + 1)
                    right -= 1
                left >>= 1
                right >>= 1

            j = nums[i] + size
            tree[j] = dp[i]
            while j > 1:
                j >>= 1
                tree[j] = max(tree[2 * j], tree[2 * j + 1])

        return max(dp)
```
