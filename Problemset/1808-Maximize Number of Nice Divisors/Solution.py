class Solution:
    def maxNiceDivisors(self, primeFactors: int) -> int:
        x, y = primeFactors // 3, primeFactors % 3

        if y == 0:
            y = 1
        elif y == 1 and x > 0:
            x -= 1
            y = 4

        return pow(3, x, 1000000007) * y % 1000000007
