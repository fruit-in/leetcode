from functools import cache


class Solution:
    def stoneGameII(self, piles: List[int]) -> int:
        @cache
        def subGame(m: int, i: int) -> int:
            if i == len(piles):
                return 0

            total = sum(piles[i:])

            return total - min(subGame(max(m, x), i + x) for x in range(1, min(2 * m, len(piles) - i) + 1))

        return subGame(1, 0)
