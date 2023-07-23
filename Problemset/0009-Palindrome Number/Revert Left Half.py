class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        if n % 2 == 1:
            left = x // (10 ** (n // 2 + 1))
        else:
            left = x // (10 ** (n // 2))
        right = x % (10 ** (n // 2))
        rev = 0
        while left != 0:
            rev *= 10
            rev += left % 10
            left //= 10
        return rev == right
