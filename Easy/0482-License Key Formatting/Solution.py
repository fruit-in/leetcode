class Solution:
    def licenseKeyFormatting(self, S: str, K: int) -> str:
        ret = ""
        cnt = 0

        for ch in S[::-1]:
            if ch == '-':
                continue

            if cnt % K == 0 and cnt != 0:
                ret = '-' + ret

            ret = ch.upper() + ret
            cnt += 1

        return ret
