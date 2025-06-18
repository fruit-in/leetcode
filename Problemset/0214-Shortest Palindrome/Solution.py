class Solution:
    def shortestPalindrome(self, s: str) -> str:
        if len(s) < 2:
            return s

        n = len(s)
        s += s[::-1]
        j = 0
        lps = [0] * (n * 2)

        for i in range(1, n * 2):
            while j > 0 and (s[i] != s[j] or j >= n):
                j = lps[j - 1]

            if s[i] == s[j]:
                j += 1
                lps[i] = j

        return s[n:n * 2 - lps[-1]] + s[:n]
