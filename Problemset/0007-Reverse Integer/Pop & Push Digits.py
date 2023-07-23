class Solution:
    def reverse(self, x: int) -> int:
        result = 0
        xc = abs(x)
        while xc != 0:
            result *= 10
            result += xc % 10
            xc //= 10
        if result < -(2 ** 31) or result > (2 ** 31) - 1:
            return 0
        elif x >= 0:
            return result
        else:
            return -result
