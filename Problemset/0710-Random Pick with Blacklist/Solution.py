import random


class Solution:

    def __init__(self, n: int, blacklist: List[int]):
        self.m = len(blacklist)
        self.n = n
        xs = set(blacklist) - set(range(self.m))
        ys = set(range(self.m)) - set(blacklist)
        self.dict = dict(zip(xs, ys))

    def pick(self) -> int:
        x = random.randint(self.m, self.n - 1)

        return self.dict.get(x, x)


# Your Solution object will be instantiated and called as such:
# obj = Solution(n, blacklist)
# param_1 = obj.pick()
