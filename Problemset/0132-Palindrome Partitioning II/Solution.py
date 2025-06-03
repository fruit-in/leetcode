class Solution:
    def minCut(self, s: str) -> int:
        @cache
        def isPalindrome(i: int, j: int) -> bool:
            return i >= j or (s[i] == s[j] and isPalindrome(i + 1, j - 1))

        @cache
        def dfs(j: int) -> int:
            if isPalindrome(0, j):
                return 0

            return min(dfs(i - 1) for i in range(1, j + 1) if isPalindrome(i, j)) + 1

        return dfs(len(s) - 1)
