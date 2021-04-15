from collections import Counter


class Solution:
    def countBalls(self, lowLimit: int, highLimit: int) -> int:
        counter = Counter(sum(map(int, str(x)))
                          for x in range(lowLimit, highLimit + 1))

        return max(counter.values())
