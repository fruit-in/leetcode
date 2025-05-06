class Solution:
    def nthUglyNumber(self, n: int, a: int, b: int, c: int) -> int:
        def leUglyCount(x: int) -> int:
            return x // a + x // b + x // c - x // d - x // e - x // f + x // g

        d, e, f, g = math.lcm(a, b), math.lcm(
            b, c),  math.lcm(a, c), math.lcm(a, b, c)

        return bisect.bisect_left(range(2000000000), n, key=leUglyCount)
