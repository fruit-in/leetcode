from functools import cache


class Solution:
    def longestPalindrome(self, word1: str, word2: str) -> int:
        @cache
        def longestSubPalindrome(i: int, j: int) -> int:
            if i > j:
                return 0
            if i == j:
                return 1

            ret = max(longestSubPalindrome(i, j - 1),
                      longestSubPalindrome(i + 1, j))
            if word[i] == word[j]:
                ret = max(ret, 2 + longestSubPalindrome(i + 1, j - 1))

            return ret

        word = word1 + word2
        first = [[-1, -1] for _ in range(26)]
        ret = 0

        for i in range(len(word1) - 1, -1, -1):
            first[ord(word1[i]) - 97][0] = i
        for i in range(len(word1), len(word)):
            first[ord(word[i]) - 97][1] = i

        for i, j in first:
            if i >= 0 and j >= 0:
                ret = max(ret, 2 + longestSubPalindrome(i + 1, j - 1))

        return ret
