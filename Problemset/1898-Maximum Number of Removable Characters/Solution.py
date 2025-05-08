class Solution:
    def maximumRemovals(self, s: str, p: str, removable: List[int]) -> int:
        def isNotSubsequence(k: int) -> int:
            removed = set(removable[:k])
            i = 0

            for j in range(len(s)):
                if j not in removed and p[i] == s[j]:
                    i += 1
                if i == len(p):
                    break

            return i < len(p)

        return bisect.bisect(range(len(removable) + 1), False, key=isNotSubsequence) - 1
