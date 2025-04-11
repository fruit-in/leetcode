from functools import cache


class Solution:
    def stoneGameIII(self, stoneValue: List[int]) -> str:
        @cache
        def scoreDiff(i: int) -> int:
            if i >= len(stoneValue):
                return 0

            score = 0
            ret = float("-inf")

            for j in range(i, min(i + 3, len(stoneValue))):
                score += stoneValue[j]
                ret = max(ret, score - scoreDiff(j + 1))

            return ret

        diff = scoreDiff(0)

        if diff > 0:
            return "Alice"
        elif diff < 0:
            return "Bob"
        else:
            return "Tie"
