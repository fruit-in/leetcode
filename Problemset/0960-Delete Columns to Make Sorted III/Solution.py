class Solution:
    def minDeletionSize(self, strs: List[str]) -> int:
        m = len(strs[0])
        dp = [[0, i] for i in range(m)]
        dp[0][0] = 1

        for i in range(1, m):
            dp[i][0] = min(dp[i - 1]) + 1
            for j in range(i):
                if all(s[j] <= s[i] for s in strs):
                    dp[i][1] = min(dp[i][1], dp[j][1] + i - j - 1)

        return min(dp[-1])
