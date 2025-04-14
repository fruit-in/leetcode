class Solution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        def val(c): return ord(c) - 96
        value = 0
        start = 0

        for i in range(len(s) - 1, -1, -1):
            value = (value * power + val(s[i])) % modulo
            if i + k < len(s):
                value = (value - val(s[i + k]) *
                         pow(power, k, modulo)) % modulo
            if value == hashValue:
                start = i

        return s[start:start + k]
