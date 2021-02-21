class Solution:
    def numRollsToTarget(self, d: int, f: int, target: int) -> int:
        if target < d or target > d * f:
            return 0

        dp = [[0] * (target + 1) for _ in range(d + 1)]
        dp[0][0] = 1

        for i in range(d):
            for j in range(max(i, target - (d - i) * f), min(i * f, target - d + i) + 1):
                for k in range(1, min(f, target - j) + 1):
                    dp[i + 1][j + k] += dp[i][j]
                    dp[i + 1][j + k] %= 1_000_000_007

        return dp[d][target]
