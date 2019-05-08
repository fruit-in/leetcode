class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False
        n = 0
        while x // (10 ** n) != 0:
            n += 1
        return self.is_palindrome(x, n - 1)
    
    def is_palindrome(self, x: int, n: int) -> bool:
        if x == 0:
            return True
        head = x // (10 ** n)
        tail = x % 10
        if head == tail:
            x %= 10 ** n
            x //= 10
            return self.is_palindrome(x, n - 2)
        else:
            return False
