class Solution:
    def splitNum(self, num: int) -> int:
        num = sorted(str(num))
        num1 = int(''.join(num[::2]))
        num2 = int(''.join(num[1::2]))

        return num1 + num2
