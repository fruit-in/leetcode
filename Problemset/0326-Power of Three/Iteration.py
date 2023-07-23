class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        while n % 3 == 0 and n != 0:
            n //= 3

        return n == 1
