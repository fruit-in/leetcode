import math


class Solution:
    def nthMagicalNumber(self, n: int, a: int, b: int) -> int:
        c = math.lcm(a, b)
        d = n // (c // a + c // b - 1)
        n %= (c // a + c // b - 1)
        l = 0
        r = c - 1
        m = (l + r) // 2

        while m // a + m // b != n or (m % a != 0 and m % b != 0):
            if m // a + m // b < n:
                l = m + 1
            else:
                r = m - 1

            m = (l + r) // 2

        return (c * d + m) % 1000000007
