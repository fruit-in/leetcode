class Solution:
    def commonChars(self, A: List[str]) -> List[str]:
        cnt1 = [100] * 26

        for wo in A:
            cnt2 = [0] * 26
            for ch in wo:
                cnt2[ord(ch) - 97] += 1

            for i in range(26):
                cnt1[i] = min(cnt1[i], cnt2[i])

        ret = []

        for i in range(26):
            ret.extend([chr(97 + i)] * cnt1[i])

        return ret
