class Solution:
    @cache
    def findIntegers(self, n: int) -> int:
        if n < 4:
            return 3 if n == 3 else n + 1

        m = n.bit_length() - 2
        ret = self.findIntegers((1 << m) - 1) + \
            self.findIntegers((1 << (m - 1)) - 1)
        if n >> m == 0b10:
            ret += self.findIntegers(n & ((1 << m) - 1))
        else:
            ret += self.findIntegers((1 << m) - 1)

        return ret
