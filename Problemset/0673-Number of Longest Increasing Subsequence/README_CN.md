# 673. 最长递增子序列的个数
给定一个未排序的整数数组 `nums` ， *返回最长递增子序列的个数* 。

**注意** 这个数列必须是 **严格** 递增的。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,5,4,7]
<strong>输出:</strong> 2
<strong>解释:</strong> 有两个最长递增子序列，分别是 [1, 3, 4, 7] 和[1, 3, 5, 7]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2,2,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 最长递增子序列的长度是1，并且存在5个子序列的长度为1，因此输出5。
</pre>

#### 提示:
* `1 <= nums.length <= 2000`
* <code>-10<sup>6</sup> <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findNumberOfLIS(self, nums: List[int]) -> int:
        dp = [[[10000001, 0], [-1000001, 1]]]

        for num in nums:
            if dp[-1][-1][0] < num:
                dp.append([[1000001, 0]])

            i = bisect.bisect_left(dp, num, key=lambda x: x[-1][0])
            j = bisect.bisect_left(dp[i - 1][::-1], num, key=lambda x: x[0])
            count = dp[i][-1][1] + dp[i - 1][-1][1] - dp[i - 1][-j - 1][1]
            dp[i].append([num, count])

        return dp[-1][-1][1]
```
