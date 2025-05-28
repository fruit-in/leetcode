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
