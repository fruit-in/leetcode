class Solution:
    def largestSumOfAverages(self, nums: List[int], k: int) -> float:
        n = len(nums)
        prefixsum = [0] * (n + 1)
        dp = [[0] * (k + 1) for _ in range(n + 1)]

        for i in range(n):
            prefixsum[i + 1] = prefixsum[i] + nums[i]

        for i in range(n):
            for j in range(min(1, i), k):
                for s in range(1, n + 1 - i):
                    dp[i + s][j + 1] = max(dp[i + s][j + 1],
                                           dp[i][j] + (prefixsum[i + s] - prefixsum[i]) / s)

        return max(dp[n])
