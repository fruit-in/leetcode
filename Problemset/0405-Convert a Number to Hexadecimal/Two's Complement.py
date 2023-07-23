class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"
        elif num > 0:
            while num > 0:
                hex_n += hex_ch[num % 16]
                num //= 16
        elif num < 0:
            num = -num
            plus_one = True

            while num > 0:
                if plus_one and num % 16 != 0:
                    hex_n += hex_ch[15 - num % 16 + 1]
                    plus_one = False
                elif plus_one:
                    hex_n += '0'
                else:
                    hex_n += hex_ch[15 - num % 16]
                num //= 16

            hex_n += 'f' * (8 - len(hex_n))

        return hex_n[::-1]
