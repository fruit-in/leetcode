class Solution:
    def longestDecomposition(self, text: str) -> int:
        @cache
        def dfs(start: int) -> int:
            n = len(text) - start * 2

            if n < 2:
                return n

            j = 0
            lps = [0] * n
            ret = 1

            for i in range(1, n):
                while j > 0 and text[start + i] != text[start + j]:
                    j = lps[j - 1]

                if text[start + i] == text[start + j]:
                    j += 1
                    lps[i] = j

            j = lps[-1] - 1
            while j >= 0:
                if text[start + n - 1] == text[start + j]:
                    ret = max(ret, 2 + dfs(start + j + 1))
                j = lps[j] - 1

            return ret

        return dfs(0)
