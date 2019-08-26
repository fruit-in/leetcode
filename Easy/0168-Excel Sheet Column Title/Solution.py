class Solution:
    def convertToTitle(self, n: int) -> str:
        ret = ""
        while n > 0:
            n -= 1
            ret = chr(n % 26 + 65) + ret
            n //= 26
        return ret
