# 2518. 好分区的数目
给你一个正整数数组 `nums` 和一个整数 `k` 。

**分区** 的定义是：将数组划分成两个有序的 **组** ，并满足每个元素 **恰好** 存在于 **某一个** 组中。如果分区中每个组的元素和都大于等于 `k` ，则认为分区是一个好分区。

返回 **不同** 的好分区的数目。由于答案可能很大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 后的结果。

如果在两个分区中，存在某个元素 `nums[i]` 被分在不同的组中，则认为这两个分区不同。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 4
<strong>输出:</strong> 6
<strong>解释:</strong> 好分区的情况是 ([1,2,3], [4]), ([1,3], [2,4]), ([1,4], [2,3]), ([2,3], [1,4]), ([2,4], [1,3]) 和 ([4], [1,2,3]) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,3,3], k = 4
<strong>输出:</strong> 0
<strong>解释:</strong> 数组中不存在好分区。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [6,6], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 可以将 nums[0] 放入第一个分区或第二个分区中。
好分区的情况是 ([6], [6]) 和 ([6], [6]) 。
</pre>

#### 提示:
* `1 <= nums.length, k <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countPartitions(self, nums: List[int], k: int) -> int:
        if sum(nums) < k * 2:
            return 0

        MOD = 1000000007
        n = len(nums)
        dp = [[0] * (k + 1) for _ in range(n + 1)]
        dp[0][0] = 1
        ret = pow(2, n, MOD)

        for i in range(n):
            for j in range(k + 1):
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % MOD
                dp[i + 1][min(k, j + nums[i])] = (dp[i + 1]
                                                  [min(k, j + nums[i])] + dp[i][j]) % MOD

        for i in range(k):
            ret = (ret - dp[n][i] * 2) % MOD

        return ret
```
