from functools import cache


class Solution:
    def palindromePartition(self, s: str, k: int) -> int:
        @cache
        def palindromeChange(i: int, j: int) -> int:
            if i >= j:
                return 0

            return palindromeChange(i + 1, j - 1) + (s[i] != s[j])

        @cache
        def subPartition(i: int, k: int) -> int:
            if k == 1:
                return palindromeChange(i, len(s) - 1)

            return min(palindromeChange(i, j) + subPartition(j + 1, k - 1) for j in range(i, len(s) - k + 1))

        return subPartition(0, k)
