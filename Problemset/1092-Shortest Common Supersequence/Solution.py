class Solution:
    def shortestCommonSupersequence(self, str1: str, str2: str) -> str:
        m, n = len(str1), len(str2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]

        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                dp[i][j] = max(dp[i + 1][j], dp[i][j + 1])
                if str1[i] == str2[j]:
                    dp[i][j] = max(dp[i][j], dp[i + 1][j + 1] + 1)

        i = 0
        j = 0
        supersequence = []

        while i < m and j < n:
            maxlcs = max(dp[i + 1][j], dp[i][j + 1])
            if str1[i] == str2[j]:
                maxlcs = max(maxlcs, dp[i + 1][j + 1] + 1)

            if maxlcs == dp[i + 1][j]:
                supersequence.append(str1[i])
                i += 1
            elif maxlcs == dp[i][j + 1]:
                supersequence.append(str2[j])
                j += 1
            else:
                supersequence.append(str1[i])
                i += 1
                j += 1

        if i == m:
            supersequence.append(str2[j:])
        if j == n:
            supersequence.append(str1[i:])

        return ''.join(supersequence)
