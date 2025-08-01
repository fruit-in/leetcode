class Solution:
    @cache
    def kSimilarity(self, s1: str, s2: str) -> int:
        chars1, chars2 = [], []
        ret = len(s1) - 1

        for i in range(len(s1)):
            if s1[i] != s2[i]:
                chars1.append(s1[i])
                chars2.append(s2[i])

        ret = max(len(chars1) - 1, 0)

        for i in range(1, len(chars1)):
            if chars1[i] == chars2[0]:
                chars1[i] = chars1[0]
                ret = min(
                    ret, 1 + self.kSimilarity(''.join(chars1[1:]), ''.join(chars2[1:])))
                chars1[i] = chars2[0]

        return ret
