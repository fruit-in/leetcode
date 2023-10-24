# 673. Number of Longest Increasing Subsequence
Given an integer array `nums`, return *the number of longest increasing subsequences*.

**Notice** that the sequence has to be **strictly** increasing.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,5,4,7]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2,2,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The length of the longest increasing subsequence is 1, and there are 5 increasing subsequences of length 1, so output 5.
</pre>

#### Constraints:
* `1 <= nums.length <= 2000`
* <code>-10<sup>6</sup> <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Python)

### 1. Solution
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
