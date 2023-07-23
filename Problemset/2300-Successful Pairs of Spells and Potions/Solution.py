import bisect


class Solution:
    def successfulPairs(self, spells: List[int], potions: List[int], success: int) -> List[int]:
        pairs = [len(potions)] * len(spells)
        potions.sort()

        for i in range(len(spells)):
            pairs[i] -= bisect.bisect_left(potions, True,
                                           key=lambda x: spells[i] * x >= success)

        return pairs
