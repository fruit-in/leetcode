class Solution:
    def maxVowels(self, s: str, k: int) -> int:
        i = 0
        cnt = 0
        ret = 0

        for j in range(len(s)):
            if s[j] in "aeiou":
                cnt += 1
            if j - i == k:
                if s[i] in "aeiou":
                    cnt -= 1
                i += 1
            ret = max(ret, cnt)

        return ret
