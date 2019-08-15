class Solution:
    def convertToBase7(self, num: int) -> str:
        if num >= 7:
            return self.convertToBase7(num // 7) + str(num % 7)
        elif num < 0:
            return '-' + self.convertToBase7(-num)
        elif num < 7:
            return str(num)
