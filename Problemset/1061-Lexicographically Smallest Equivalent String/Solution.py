class Solution:
    def smallestEquivalentString(self, s1: str, s2: str, baseStr: str) -> str:
        equivalent = list(range(26))

        for ch1, ch2 in zip(s1, s2):
            ch1, ch2 = ord(ch1) - 97, ord(ch2) - 97

            while equivalent[equivalent[ch1]] != equivalent[ch1]:
                equivalent[ch1] = equivalent[equivalent[ch1]]
            while equivalent[equivalent[ch2]] != equivalent[ch2]:
                equivalent[ch2] = equivalent[equivalent[ch2]]

            if equivalent[ch1] < equivalent[ch2]:
                equivalent[equivalent[ch2]] = equivalent[ch1]
            else:
                equivalent[equivalent[ch1]] = equivalent[ch2]

        for ch in range(26):
            while equivalent[equivalent[ch]] != equivalent[ch]:
                equivalent[ch] = equivalent[equivalent[ch]]

        return ''.join(chr(equivalent[ord(ch) - 97] + 97) for ch in baseStr)
