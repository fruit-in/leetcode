class Solution:
    aseqpositions = [(1 << i) - 1 for i in range(15)]

    @cache
    def racecar(self, target: int, reverse: bool = False) -> int:
        ret = inf

        for i in range(15):
            if not reverse and self.aseqpositions[i] == target:
                return i

            for j in range(i - 1, -1, -1):
                position = self.aseqpositions[i] - self.aseqpositions[j]

                if position < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(target - position, reverse))
                elif position == target:
                    ret = min(ret, i + j + 1)
                elif position - target < target:
                    ret = min(ret, i + j + 2 +
                              self.racecar(position - target, not reverse))
                else:
                    break

        return ret
