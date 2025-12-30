class Solution:
    @cache
    def maxCheck(self, pigs: int, rounds: int) -> int:
        if pigs == 0:
            return 1
        if pigs == 1:
            return rounds + 1
        if rounds == 1:
            return 2 ** pigs

        ret = 0

        for p in range(0, pigs + 1):
            ret += comb(pigs, p) * self.maxCheck(pigs - p, rounds - 1)

        return ret

    def poorPigs(self, buckets: int, minutesToDie: int, minutesToTest: int) -> int:
        pigs = ceil(log2(buckets))
        rounds = minutesToTest // minutesToDie

        for p in range(0, pigs):
            if self.maxCheck(p, rounds) >= buckets:
                return p

        return pigs
