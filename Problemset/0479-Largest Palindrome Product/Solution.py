class Solution:
    def largestPalindrome(self, n: int) -> int:
        if n == 1:
            return 9

        maxx, minx = 10 ** n - 1, 10 ** (n - 1)
        for x in range(int(str(maxx ** 2)[:n]), minx - 1, -1):
            palindrome = int(str(x) + str(x)[::-1])
            for y in range(maxx, int(sqrt(palindrome)) - 1, -1):
                if palindrome % y == 0:
                    return palindrome % 1337
