from functools import cache
from itertools import permutations
from math import gcd


class Solution:
    @cache
    def add(self, a: (int, int), b: (int, int)) -> (int, int):
        lcm = abs(a[1] * b[1]) // gcd(a[1], b[1])
        return (a[0] * lcm // a[1] + b[0] * lcm // b[1], lcm)

    @cache
    def sub(self, a: (int, int), b: (int, int)) -> (int, int):
        return self.add(a, (-b[0], b[1]))

    @cache
    def mul(self, a: (int, int), b: (int, int)) -> (int, int):
        return (a[0] * b[0], a[1] * b[1])

    @cache
    def div(self, a: (int, int), b: (int, int)) -> (int, int):
        if b[0] == 0:
            return (0, 1)
        return self.mul(a, (b[1], b[0]))

    @cache
    def judge1(self, a: (int, int)) -> bool:
        return a[0] % a[1] == 0 and a[0] // a[1] == 24

    @cache
    def judge2(self, a: (int, int), b: (int, int)) -> bool:
        return self.judge1(self.add(a, b)) or self.judge1(self.sub(a, b)) or self.judge1(self.mul(a, b)) or self.judge1(self.div(a, b))

    @cache
    def judge3(self, a: (int, int), b: (int, int), c: (int, int)) -> bool:
        return self.judge2(self.add(a, b), c) or self.judge2(self.sub(a, b), c) or self.judge2(self.mul(a, b), c) or self.judge2(self.div(a, b), c) or \
            self.judge2(a, self.add(b, c)) or self.judge2(a, self.sub(b, c)) or self.judge2(
                a, self.mul(b, c)) or self.judge2(a, self.div(b, c))

    @cache
    def judge4(self, a: (int, int), b: (int, int), c: (int, int), d: (int, int)) -> bool:
        return self.judge3(self.add(a, b), c, d) or self.judge3(self.sub(a, b), c, d) or self.judge3(self.mul(a, b), c, d) or self.judge3(self.div(a, b), c, d) or \
            self.judge3(a, self.add(b, c), d) or self.judge3(a, self.sub(b, c), d) or self.judge3(a, self.mul(b, c), d) or self.judge3(a, self.div(b, c), d) or \
            self.judge3(a, b, self.add(c, d)) or self.judge3(a, b, self.sub(
                c, d)) or self.judge3(a, b, self.mul(c, d)) or self.judge3(a, b, self.div(c, d))

    def judgePoint24(self, cards: List[int]) -> bool:
        for (a, b, c, d) in permutations(cards):
            if self.judge4((a, 1), (b, 1), (c, 1), (d, 1)):
                return True
        return False
