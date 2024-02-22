class Solution:
    def numberOfArrays(self, s: str, k: int) -> int:
        dp = [0] * (len(s) + 1)
        dp[0] = 1

        for i in range(len(s)):
            if s[i] == '0':
                continue

            x = int(s[i])
            j = 1

            while x <= k and i + j <= len(s):
                dp[i + j] = (dp[i + j] + dp[i]) % 1000000007
                if i + j < len(s):
                    x = x * 10 + int(s[i + j])
                j += 1

        return dp[-1]
