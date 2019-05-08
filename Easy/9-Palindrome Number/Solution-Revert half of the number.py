class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        rev = 0
        for i in range(n // 2):
            rev *= 10
            rev += x % 10
            x //= 10
        return rev == x or rev == (x // 10)
