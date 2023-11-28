class Solution:
    def peopleAwareOfSecret(self, n: int, delay: int, forget: int) -> int:
        dp = [[0, 0, 0] for _ in range(n + 2)]
        dp[1 + forget][0] = 1
        dp[1 + delay][1] = 1
        dp[1][2] = 1

        for i in range(2, n + 1):
            if i > 1 + forget:
                dp[i][0] = dp[i - forget][1]
            if i > 1 + delay:
                dp[i][1] = dp[i - 1][1] - dp[i][0] + dp[i - delay][1]
            dp[i][2] = dp[i - 1][2] - dp[i][0] + dp[i][1]

        return dp[n][2] % 1000000007
