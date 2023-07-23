class Solution:
    def numberOfSets(self, n: int, k: int) -> int:
        dp = [[0] * (n + 1) for _ in range(k + 1)]

        for j in range(2, n + 1):
            dp[1][j] = dp[1][j - 1] + j - 1

        for i in range(2, k + 1):
            for j in range(i + 1, n + 1):
                dp[i][j] = dp[i - 1][j - 1] + 2 * dp[i][j - 1] - dp[i][j - 2]

        return dp[k][n] % 1000000007
