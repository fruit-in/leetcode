import random


class Solution:

    def __init__(self, rects: List[List[int]]):
        self.rects = []
        self.start = 0

        for a, b, x, y in rects:
            self.rects.append((self.start, a, b, x, y))
            self.start += (x - a + 1) * (y - b + 1)

    def pick(self) -> List[int]:
        n = random.randint(0, self.start - 1)
        i = bisect.bisect(self.rects, n, key=lambda r: r[0]) - 1
        start, a, b, x, _ = self.rects[i]
        n -= start

        return [a + n % (x - a + 1), b + n // (x - a + 1)]


# Your Solution object will be instantiated and called as such:
# obj = Solution(rects)
# param_1 = obj.pick()
