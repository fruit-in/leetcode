class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"

        if num < 0:
            num &= 0x7fffffff
            hex_n += hex_ch[num & 0xf]
            num >>= 4
            num |= 0x08000000

        while num != 0:
            hex_n += hex_ch[num & 0xf]
            num >>= 4

        return hex_n[::-1]
