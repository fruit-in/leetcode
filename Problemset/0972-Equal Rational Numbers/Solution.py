class Solution:
    def isRationalEqual(self, s: str, t: str) -> bool:
        def toFraction(s: str) -> (int, int):
            s = re.split(r"[.()]", s)
            integer = int(s[0])
            nonrepeating = int(s[1]) if len(s) > 1 and s[1] != '' else None
            repeating = int(s[2]) if len(s) > 2 else None

            if nonrepeating is None and repeating is None:
                denominator = 1
                numerator = 0
            elif nonrepeating is None:
                denominator = 10 ** len(s[2]) - 1
                numerator = repeating
            elif repeating is None:
                denominator = 10 ** len(s[1])
                numerator = nonrepeating
            else:
                denominator = 10 ** (len(s[1]) + len(s[2])) - 10 ** (len(s[1]))
                numerator = nonrepeating * (10 ** len(s[2]) - 1) + repeating

            numerator += denominator * integer
            gcdnum = gcd(numerator, denominator)

            return (numerator // gcdnum, denominator // gcdnum)

        return toFraction(s) == toFraction(t)
