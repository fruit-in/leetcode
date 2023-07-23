class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"
        elif num < 0:
            num = 4294967296 + num

        while num > 0:
            hex_n += hex_ch[num % 16]
            num //= 16

        return hex_n[::-1]
