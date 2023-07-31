class Solution:
    def makeIntegerBeautiful(self, n: int, target: int) -> int:
        a = 1
        m = n

        while sum(int(ch) for ch in str(m)) > target:
            m += (10 - m // a % 10) * a
            a *= 10

        return m - n
