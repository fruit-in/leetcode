class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        if n % 2 == 1:
            part1 = x // (10 ** (n // 2 + 1))
            part2 = x % (10 ** (n // 2))
        else:
            part1 = x // (10 ** (n // 2))
            part2 = x % (10 ** (n // 2))
        rev = 0
        while part1 != 0:
            rev *= 10
            rev += part1 % 10
            part1 //= 10
        return rev == part2
