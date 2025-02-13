from random import randint


class Solution:

    def __init__(self, m: int, n: int):
        self.m = m
        self.n = n
        self.remain = m * n
        self.mapping = {}

    def flip(self) -> List[int]:
        self.remain -= 1
        i = randint(0, self.remain)
        self.mapping[i], self.mapping[self.remain] = self.mapping.get(
            self.remain, self.remain), self.mapping.get(i, i)

        return [self.mapping[self.remain] // self.n, self.mapping[self.remain] % self.n]

    def reset(self) -> None:
        self.remain = self.m * self.n
        self.mapping.clear()


# Your Solution object will be instantiated and called as such:
# obj = Solution(m, n)
# param_1 = obj.flip()
# obj.reset()
