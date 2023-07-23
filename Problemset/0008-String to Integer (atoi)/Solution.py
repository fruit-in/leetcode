class Solution:
    def myAtoi(self, str: str) -> int:
        str = str.lstrip()
        sign = -1 if str and str[0] == '-' else 0
        n = 0

        if str and (str[0] == '+' or str[0] == '-'):
            str = str[1:]

        for i in range(len(str)):
            if str[i].isdigit():
                x = ord(str[i]) - 48
                if n > 214748364 or (n == 214748364 and x + sign > 7):
                    return 2147483647 if sign == 0 else -2147483648
                n *= 10
                n += x
            else:
                break

        return n if sign == 0 else -n
