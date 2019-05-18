class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        n -= 1
        while x != 0:
            head = x // (10 ** n)
            tail = x % 10
            if head == tail:
                x %= 10 ** n
                x //= 10
                n -= 2
            else:
                return False
        return True
