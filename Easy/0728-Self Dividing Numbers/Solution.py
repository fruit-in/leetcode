class Solution:
    def selfDividingNumbers(self, left: int, right: int) -> List[int]:
        return list(filter(self.isSelfDividingNumber, range(left, right + 1)))
    
    def isSelfDividingNumber(self, num: int) -> bool:
        if num == 0:
            return False
        n = num
        while num != 0:
            if num % 10 == 0 or n % (num % 10) != 0:
                return False
            num //= 10
        return True
