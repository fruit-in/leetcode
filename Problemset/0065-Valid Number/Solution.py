class Solution:
    def isNumber(self, s: str) -> bool:
        def isDigits(s: str) -> bool:
            return s != "" and all(c in "0123456789" for c in s)

        def isInteger(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]

            return isDigits(s)

        def isDecimal(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]
            intpart, _, decipart = s.partition('.')

            if isDigits(intpart) and decipart == "":
                return True
            if isDigits(decipart) and intpart == "":
                return True

            return isDigits(intpart) and isDigits(decipart)

        num, e, exp = s.lower().partition('e')

        return (isInteger(num) or isDecimal(num)) and (e == "" or isInteger(exp))
