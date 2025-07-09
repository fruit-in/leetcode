# 2407. Longest Increasing Subsequence II
You are given an integer array `nums` and an integer `k`.

Find the longest subsequence of `nums` that meets the following requirements:
* The subsequence is **strictly increasing** and
* The difference between adjacent elements in the subsequence is **at most** `k`.

Return *the length of the **longest subsequence** that meets the requirements*.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,2,1,4,3,4,5,8,15], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The longest subsequence that meets the requirements is [1,3,4,5,8].
The subsequence has a length of 5, so we return 5.
Note that the subsequence [1,3,4,5,8,15] does not meet the requirements because 15 - 8 = 7 is larger than 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [7,4,5,1,8,12,4,7], k = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The longest subsequence that meets the requirements is [4,5,8,12].
The subsequence has a length of 4, so we return 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,5], k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The longest subsequence that meets the requirements is [1].
The subsequence has a length of 1, so we return 1.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], k <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
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
