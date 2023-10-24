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
