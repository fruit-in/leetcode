class Solution:
    def countAndSay(self, n: int) -> str:
        s = "1"
        for i in range(n - 1):
            tmp = ""
            i = 0
            for j in range(len(s)):
                if s[i] != s[j]:
                    tmp += str(j - i) + s[i]
                    i = j
            tmp += str(len(s) - i) + s[i]
            s = tmp
        return s
